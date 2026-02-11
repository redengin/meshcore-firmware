#![no_std]

pub use log;
pub use esp_println;

pub use embassy_executor;
// pub use embassy_futures;
pub use embassy_time::{Duration, Timer};

pub use esp_rtos;
pub use esp_radio;
pub use esp_alloc;
pub use trouble_host;


use log::*;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // display location
    if let Some(location) = info.location() {
        error!(
            "PANIC at {} {}:{}",
            location.file(),
            location.line(),
            location.column()
        );
    }
    // display message
    error!("{}", info.message());

    loop {
        // wait for logging message to publish
        let delay = esp_hal::delay::Delay::new();
        delay.delay_millis(1000);

        // if release build, reset to resume mission
        #[cfg(not(debug_assertions))]
        esp_hal::system::software_reset()
    }
}

#[macro_export]
macro_rules! create_heap {
    // provide 64K heap (reclaimed from bootloader)
    () => {
        const BOOTLOADER_RAM_SZ: usize = 64 * 1024;
        crate::esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: BOOTLOADER_RAM_SZ);
    };
}

/// initialize the SoC
/// * initialize logging
/// * provide BLE and WiFi support
pub fn init() -> (esp_hal::peripherals::Peripherals) {
    let peripherals = esp_hal::init(
        // max out clock to support radio
        esp_hal::Config::default().with_cpu_clock(esp_hal::clock::CpuClock::max()),
    );

    return peripherals;
}
