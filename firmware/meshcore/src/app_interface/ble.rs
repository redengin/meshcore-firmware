// provide logging primitives
use log::*;

// provide scheduling primitives
use embassy_futures::{join::join, select::select};
use embassy_time::Timer;


/// convert MAC to a string "MeshCore-XXXXXXXXXXXX"
/// TODO move this to common method
fn name_from_mac(mac: [u8; 6]) -> heapless::String<32> {
    let mut ssid: heapless::String<32> = heapless::String::try_from("MeshCore-").unwrap();
    // add the MAC address
    for byte in mac {
        ssid.push(hex_char(byte / 16)).unwrap();
        ssid.push(hex_char(byte % 16)).unwrap();
    }
    return ssid;

    fn hex_char(val: u8) -> char {
        if val < 10 {
            return (('0' as u8) + val) as char;
        };
        return (('A' as u8) + val - 10) as char;
    }
}

/// Run the BLE stack.
pub async fn run<C>(controller: C, mac: [u8; 6])
where
    C: trouble_host::Controller,
{
    /// Max number of connections
    const CONNECTIONS_MAX: usize = 1;
    /// Max number of L2CAP channels.
    const L2CAP_CHANNELS_MAX: usize = 2; // Signal + att

    // create resources
    use trouble_host::{HostResources, prelude::DefaultPacketPool};
    let mut resources: HostResources<DefaultPacketPool, CONNECTIONS_MAX, L2CAP_CHANNELS_MAX> =
        HostResources::new();

    // create the stack
    use trouble_host::{Address, Host};
    let address: Address = Address::random(mac);
    let stack = trouble_host::new(controller, &mut resources).set_random_address(address);
    let Host {
        mut peripheral,
        runner,
        ..
    } = stack.build();

    // create the BLE server
    let _name = name_from_mac(mac);
    let server = Server::new_with_config(GapConfig::Peripheral(PeripheralConfig {
        // name: &name,
        name: "TrouBLE",
        // appearance: &appearance::network_device::MESH_DEVICE,
        appearance: &appearance::power_device::GENERIC_POWER_DEVICE,
    }))
    .unwrap();

    // run the BLE host service (advertise -> connect -> serve -disconnect-> advertise)
    let _ = join(ble_task(runner), async {
        loop {
            match advertise("Trouble Example", &mut peripheral, &server).await {
                Ok(conn) => {
                    // set up tasks when the connection is established to a central, so they don't run when no one is connected.
                    let a = gatt_events_task(&server, &conn);
                    let b = custom_task(&server, &conn, &stack);
                    // run until any task ends (usually because the connection has been closed),
                    // then return to advertising state.
                    select(a, b).await;
                }
                Err(e) => {
                    #[cfg(feature = "defmt")]
                    let e = defmt::Debug2Format(&e);
                    // panic!("[adv] error: {:?}", e);
                }
            }
        }
    })
    .await;
}

/// This is a background task that is required to run forever alongside any other BLE tasks.
/// ## Alternative
/// ```
/// spawner.must_spawn(ble_task(runner));
/// ...
/// #[embassy_executor::task]
/// async fn ble_task(mut runner: Runner<'static, SoftdeviceController<'static>>) {
///     runner.run().await;
/// }
/// ```
async fn ble_task<C: Controller, P: PacketPool>(mut runner: Runner<'_, C, P>) {
    loop {
        if let Err(e) = runner.run().await {
            #[cfg(feature = "defmt")]
            let e = defmt::Debug2Format(&e);
            panic!("[ble_task] error: {:?}", e);
        }
    }
}

/// Create an advertiser to use to connect to a BLE Central, and wait for it to connect.
async fn advertise<'values, 'server, C: Controller>(
    name: &'values str,
    peripheral: &mut Peripheral<'values, C, DefaultPacketPool>,
    server: &'server Server<'values>,
) -> Result<GattConnection<'values, 'server, DefaultPacketPool>, BleHostError<C::Error>> {
    let mut advertiser_data = [0; 31];
    let len = AdStructure::encode_slice(
        &[
            AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
            AdStructure::ServiceUuids16(&[[0x0f, 0x18]]),
            AdStructure::CompleteLocalName(name.as_bytes()),
        ],
        &mut advertiser_data[..],
    )?;
    let advertiser = peripheral
        .advertise(
            &Default::default(),
            Advertisement::ConnectableScannableUndirected {
                adv_data: &advertiser_data[..len],
                scan_data: &[],
            },
        )
        .await?;
    info!("[adv] advertising");
    let conn = advertiser.accept().await?.with_attribute_server(server)?;
    info!("[adv] connection established");
    Ok(conn)
}

/// Handle GATT Events until the connection closes
async fn gatt_events_task<P: PacketPool>(server: &Server<'_>, conn: &GattConnection<'_, '_, P>) -> Result<(), Error> {
    // let level = server.battery_service.level;
    let reason = loop {
        match conn.next().await {
            GattConnectionEvent::Disconnected { reason } => break reason,
            GattConnectionEvent::Gatt { event } => {
                match &event {
                    // FIXME handle GATT events
                    // GattEvent::Read(event) => {
                    //     if event.handle() == level.handle {
                    //         let value = server.get(&level);
                    //         info!("[gatt] Read Event to Level Characteristic: {:?}", value);
                    //     }
                    // }
                    // GattEvent::Write(event) => {
                    //     if event.handle() == level.handle {
                    //         info!("[gatt] Write Event to Level Characteristic: {:?}", event.data());
                    //     }
                    // }
                    _ => {}
                };
                // This step is also performed at drop(), but writing it explicitly is necessary
                // in order to ensure reply is sent.
                match event.accept() {
                    Ok(reply) => reply.send().await,
                    Err(e) => warn!("[gatt] error sending response: {:?}", e),
                };
            }
            _ => {} // ignore other Gatt Connection Events
        }
    };
    info!("[gatt] disconnected: {:?}", reason);
    Ok(())
}

/// Example task to use the BLE notifier interface.
/// This task will notify the connected central of a counter value every 2 seconds.
/// It will also read the RSSI value every 2 seconds.
/// and will stop when the connection is closed by the central or an error occurs.
async fn custom_task<C: Controller, P: PacketPool>(
    server: &Server<'_>,
    conn: &GattConnection<'_, '_, P>,
    stack: &Stack<'_, C, P>,
) {
    let mut tick: u8 = 0;
    // let level = server.battery_service.level;
    loop {
        tick = tick.wrapping_add(1);
        info!("[custom_task] notifying connection of tick {}", tick);
        // if level.notify(conn, &tick).await.is_err() {
        //     info!("[custom_task] error notifying connection");
        //     break;
        // };
        // read RSSI (Received Signal Strength Indicator) of the connection.
        if let Ok(rssi) = conn.raw().rssi(stack).await {
            info!("[custom_task] RSSI: {:?}", rssi);
        } else {
            info!("[custom_task] error getting RSSI");
            break;
        };
        Timer::after_secs(2).await;
    }
}


// GATT Server definition
//==============================================================================
const BLE_MTU_MAX: usize = 1024;
use trouble_host::prelude::*;
#[gatt_server]
struct Server {
    meshcore_service: MeshCoreService,
}

/// BLE Service per https://github.com/meshcore-dev/MeshCore/blob/main/docs/companion_protocol.md
#[gatt_service(uuid = "6E400001-B5A3-F393-E0A9-E50E24DCCA9E")]
struct MeshCoreService {
    // support for serial interface protocol
    //------------------------------------------------------------------------------
    // Rx (App -> Firmware)
    #[characteristic(uuid = "6E400002-B5A3-F393-E0A9-E50E24DCCA9E", write, notify)]
    pub rx: heapless::Vec<u8, BLE_MTU_MAX>,

    // Tx (Firmware -> App)
    #[characteristic(uuid = "6E400003-B5A3-F393-E0A9-E50E24DCCA9E", read, notify)]
    pub tx: heapless::Vec<u8, BLE_MTU_MAX>,
    //------------------------------------------------------------------------------

    // TODO extend to provide true BLE interface
    // #[descriptor(uuid = descriptors::VALID_RANGE, read, value = [0, 100])]
    // #[descriptor(uuid = descriptors::MEASUREMENT_DESCRIPTION, name = "hello", read, value = "Battery Level", type = &'static str)]
    // #[characteristic(uuid = characteristic::BATTERY_LEVEL, read, notify, value = 10)]
    // battery_level: u8,
}
//==============================================================================
