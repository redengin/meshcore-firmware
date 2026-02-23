// provide logging primitives
use log::*;
const TAG: &str = "[BLE]";

// provide scheduling primitives
use embassy_futures::{join::join, select::select};
use embassy_time::Timer;

/// convert MAC to a string "MeshCore-XXXXXXXXXXXX"
/// TODO move this to common method
fn name_from_mac(mac: [u8; 6]) -> heapless::String<32> {
    let mut ssid: heapless::String<32> = heapless::String::try_from("MeshCore-").unwrap();
    // add the MAC address
    // FIXME currently in reversed order
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
pub async fn run<C, RNG>(controller: C, mac: [u8; 6], random_generator: &mut RNG)
where
    C: trouble_host::Controller,
    RNG: rand_core::RngCore + rand_core::CryptoRng,
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
    let stack = trouble_host::new(controller, &mut resources)
        .set_random_address(address)
        .set_random_generator_seed(random_generator);
    // require pin code entry from central
    stack.set_io_capabilities(IoCapabilities::DisplayOnly);
    let Host {
        mut peripheral,
        runner,
        ..
    } = stack.build();

    // create the BLE server
    let server = Server::new_with_config(GapConfig::Peripheral(PeripheralConfig {
        name: "MeshCore BLE", // internal identifier (not published)
        appearance: &appearance::network_device::MESH_DEVICE, // FIXME this doesnt' appaer to be published in advertisement
    }))
    .unwrap();

    // run the BLE host service (advertise -> connect -> serve -disconnect-> advertise)
    let name = name_from_mac(mac);
    let _ = join(ble_task(runner), async {
        loop {
            match advertise(&name, &mut peripheral, &server).await {
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
                    panic!("{TAG} error: {:?}", e);
                }
            }
        }
    })
    .await;
}

/// This is a background task that is required to run forever alongside any other BLE tasks.
async fn ble_task<C: Controller, P: PacketPool>(mut runner: Runner<'_, C, P>) {
    loop {
        if let Err(e) = runner.run().await {
            #[cfg(feature = "defmt")]
            let e = defmt::Debug2Format(&e);
            panic!("{TAG} error: {:?}", e);
        }
    }
}

/// Create an advertiser to use to connect to a BLE Central, and wait for it to connect.
async fn advertise<'values, 'server, C: Controller>(
    name: &str,
    peripheral: &mut Peripheral<'values, C, DefaultPacketPool>,
    server: &'server Server<'values>,
) -> Result<GattConnection<'values, 'server, DefaultPacketPool>, BleHostError<C::Error>> {
    let mut advertiser_data = [0; 31];
    const SERVICE_UUID_MESH_PROXY_LE: [u8; 2] = [0x28, 0x18];
    let len = AdStructure::encode_slice(
        &[
            AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
            AdStructure::ServiceUuids16(&[SERVICE_UUID_MESH_PROXY_LE]),
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
    info!("{TAG} advertising as {name}");
    let conn = advertiser.accept().await?.with_attribute_server(server)?;
    info!("{TAG} connection established");
    Ok(conn)
}

/// Handle GATT Events until the connection closes
async fn gatt_events_task<P: PacketPool>(
    server: &Server<'_>,
    connection: &GattConnection<'_, '_, P>,
) -> Result<(), Error> {
    // let level = server.battery_service.level;
    let reason = loop {
        match connection.next().await {
            GattConnectionEvent::PassKeyDisplay(key) => {
                info!("{TAG} pin code {key}");
                // TODO pass this pin code to the display
                // alternative - dictate the pin code per method parameters
            }

            GattConnectionEvent::Gatt { event } => {
                // handle GATT events
                match &event {
                    GattEvent::Write(event) => {
                        let event_handle = event.handle();
                        // writable GATT attributes
                        let gatt_rx = &server.meshcore_v1.rx;
                        // match event handle to GATT attribute
                        if event_handle == gatt_rx.handle {
                            let result = server.get(gatt_rx);
                            match result {
                                Ok(data) => {
                                    // TODO process the data via the codec

                                    // respond via the tx attribute
                                    let _ = server.meshcore_v1.tx.notify(connection, &data).await;
                                }
                                Err(e) => warn!("{TAG} failed tx data read [error: {:?}", e),
                            }
                        } else {
                            warn!("{TAG} ignored gatt write of [handle: {event_handle}]");
                        }
                    }

                    // GattEvent::Read(event) => {
                    //     let event_handle = event.handle();
                    //     // readable GATT attributes
                    //     let gatt_tx = &server.meshcore_v1.tx;
                    //     // match event handle to GATT attribute
                    //     if event_handle == gatt_tx.handle {
                    //         let result = server.get(gatt_tx);
                    //         match result {
                    //             Ok(data) => {
                    //                 // TODO send the tx result
                    //             }
                    //             Err(e) => warn!("{TAG} failed read of tx data [error: {:?}", e),
                    //         }
                    //     } else {
                    //         warn!("{TAG} ignored gatt read of [handle: {event_handle}]");
                    //     }
                    // }

                    _ => warn!("{TAG} unhandled GattEvent")
                    // FIXME identify what event was missed
                    // _ => {
                    //     warn!("{TAG} unhandled GattEvent [event {:?}", event.type_id())
                    // }
                };
                // This step is also performed at drop(), but writing it explicitly is necessary
                // in order to ensure reply is sent.
                match event.accept() {
                    Ok(reply) => reply.send().await,
                    Err(e) => warn!("{TAG} gatt error sending response: {:?}", e),
                };
            }

            GattConnectionEvent::Disconnected { reason } => break reason,
            _ => {} // ignore other Gatt Connection Events
        }
    };
    info!("{TAG} disconnected: {:?}", reason);
    Ok(())
}

/// task to use the BLE notifier interface
async fn custom_task<C: Controller, P: PacketPool>(
    _server: &Server<'_>,
    conn: &GattConnection<'_, '_, P>,
    stack: &Stack<'_, C, P>,
) {
    loop {
        if let Ok(rssi) = conn.raw().rssi(stack).await {
            debug!("{TAG} RSSI: {:?}", rssi);
        } else {
            error!("{TAG} error getting RSSI");
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
    meshcore_v1: MeshCoreService,
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
