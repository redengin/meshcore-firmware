#![no_std]
#![no_main]

// provide panic handler
use soc_esp32::{self as _};
// use soc_esp32::esp_backtrace as _;  // use the esp32 supplied panic handler

// provide logging primitives
use soc_esp32::log::*;

// provide the esp_hal via re-export
use soc_esp32::*;

// provice scheduling primitives
// use embassy_time::{Duration, Timer};

#[soc_esp32::esp_rtos::main]
// async fn main(spawner: soc_esp32::embassy_executor::Spawner) -> ! {
async fn main(spawner: embassy_executor::Spawner) -> ! {
    // TODO move this into an soc_esp32::init()
    //------------------------------------------------------------------------------
    // initialize the SoC interface
    let peripherals = esp_hal::init(
        // max out clock to support radio
        esp_hal::Config::default().with_cpu_clock(esp_hal::clock::CpuClock::max()),
    );

    // initialize logging
    esp_println::logger::init_logger_from_env();
    info!("initializing...");

    // initialize the rtos
    use esp_hal::timer::timg::TimerGroup;
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    use esp_hal::interrupt::software::SoftwareInterruptControl;
    let sw_int = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_int.software_interrupt0);

    // initialize the bluetooth hardware
    // https://github.com/esp-rs/esp-hal/tree/main/examples/ble/bas_peripheral
    create_heap!(); // required by radio (use 64K reclaimed from bootloader)
    let ble_connector = esp_radio::ble::controller::BleConnector::new(
        peripherals.BT,
        esp_radio::ble::Config::default().with_max_connections(1),
    )
    .unwrap();
    info!("BLE initialized");
    spawner.spawn(task_ble_host(ble_connector)).unwrap();

    // initialize WiFi hardware
    // https://github.com/esp-rs/esp-hal/blob/main/examples/wifi/80211_tx/
    let (mut wifi_controller, _interfaces) =
        esp_radio::wifi::new(peripherals.WIFI, Default::default()).unwrap();
    wifi_controller
        .set_mode(esp_radio::wifi::WifiMode::Station)
        .unwrap();
    wifi_controller.start_async().await.unwrap();
    info!("WiFi initialized");

    //------------------------------------------------------------------------------

    loop {
        info!("Hello world!");
        Timer::after(Duration::from_secs(1)).await;
    }
}

#[embassy_executor::task]
async fn task_ble_host(ble_connector: esp_radio::ble::controller::BleConnector<'static>) {
    /// Max number of connections
    const CONNECTIONS_MAX: usize = 1;
    /// Max number of L2CAP channels.
    const L2CAP_CHANNELS_MAX: usize = 2; // Signal + att

    // create the BLE Host controller (i.e trouble_host)
    use trouble_host::prelude::*;
    let ble_controller: ExternalController<_, 1> = ExternalController::new(ble_connector);

    // configure trouble_host
    let mut resources: HostResources<DefaultPacketPool, CONNECTIONS_MAX, L2CAP_CHANNELS_MAX> =
        HostResources::new();
    let mac = esp_hal::efuse::Efuse::mac_address();
    let stack =
        trouble_host::new(ble_controller, &mut resources).set_random_address(Address::random(mac));
    let Host {
        mut peripheral,
        runner,
        ..
    } = stack.build();

    info!("Starting advertising and GATT service");
    // TODO implement


    error!("BLE host stopped");
}
