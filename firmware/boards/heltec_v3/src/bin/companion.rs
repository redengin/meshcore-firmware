#![no_std]
#![no_main]

// provide panic handler
use soc_esp32::{self as _};
// use soc_esp32::esp_backtrace as _;  // use the esp32 supplied panic handler

// provide the shared crates via re-export
use common::*;
use soc_esp32::*;

// provide logging primitives
use log::*;

// provice scheduling primitives
// use embassy_time::{Duration, Timer};

#[soc_esp32::esp_rtos::main]
// async fn main(spawner: soc_esp32::embassy_executor::Spawner) -> ! {
async fn main(_spawner: embassy_executor::Spawner) -> ! {
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
    create_heap!(); // required by radio (use default 64K heap)
    // TODO initialize bluetooth

    loop {
        // info!("Hello world!");
        // Timer::after(Duration::from_secs(1)).await;
    }
}

// #[embassy_executor::task]
// async fn task_modulator() -> ! {
//     loop {
//         info!("modulating");
//         Timer::after(Duration::from_secs(1)).await;
//     }
// }
