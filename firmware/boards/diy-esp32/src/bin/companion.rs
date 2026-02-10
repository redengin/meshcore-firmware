#![no_std]
#![no_main]

// provide panic handler
use soc_esp32::{self as _};
// use soc_esp32::esp_backtrace as _;  // use the esp32 supplied panic handler

// provide logging primitives
use soc_esp32::log::*;

// provide the esp_hal via re-export
use soc_esp32::{*};

// provice scheduling primitives
// use embassy_time::{Duration, Timer};

#[soc_esp32::esp_rtos::main]
// async fn main(spawner: soc_esp32::embassy_executor::Spawner) -> ! {
async fn main(_spawner: embassy_executor::Spawner) -> ! {
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
        esp_radio::ble::Config::default()
            .with_max_connections(1)
    ).unwrap();
    // FIXME need a BLE service
    // let ble_controller = esp_radio::ble::controller::ExternalController::new(ble_connector);
    info!("BLE initialized");

    // initialize WiFi hardware
    // https://github.com/esp-rs/esp-hal/blob/main/examples/wifi/80211_tx/
    let (mut controller, interfaces) =
        esp_radio::wifi::new(peripherals.WIFI, Default::default()).unwrap();
    controller.set_mode(esp_radio::wifi::WifiMode::Station).unwrap();
    controller.start_async().await.unwrap();
    info!("WiFi initialized");

//------------------------------------------------------------------------------

    loop {
        info!("Hello world!");
        Timer::after(Duration::from_secs(1)).await;
    }
}

// #[embassy_executor::task]
// async fn task_modulator() -> ! {
//     loop {
//         info!("modulating");
//         Timer::after(Duration::from_secs(1)).await;
//     }
// }
