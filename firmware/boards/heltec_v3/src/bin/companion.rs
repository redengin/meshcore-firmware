#![no_std]
#![no_main]

// provide panic handler
use soc_esp32::{self as _};
// use esp_backtrace as _;  // use the esp32 supplied panic handler

// provide logging primitives
use soc_esp32::log::{*};

// provide the esp_hal via re-export
// use soc_esp32::{*};
use soc_esp32::{*};

// provide heap allocator
// use sonic_reducer_esp32::{create_heap};

// provice scheduling primitives
// use embassy_time::{Duration, Timer};

#[soc_esp32::esp_rtos::main]
// async fn main(spawner: soc_esp32::embassy_executor::Spawner) -> ! {
async fn main(spawner: embassy_executor::Spawner) -> ! {
    // initialize the SoC interface
    let peripherals = esp_hal::init(
        // max out clock to support radio
        esp_hal::Config::default().with_cpu_clock(esp_hal::clock::CpuClock::max()),
    );

    // initialize logging
    esp_println::logger::init_logger_from_env();
    info!("initializing...");

    // // initialize the rtos
    // use esp_hal::timer::timg::TimerGroup;
    // let timg0 = TimerGroup::new(peripherals.TIMG0);
    // use esp_hal::interrupt::software::SoftwareInterruptControl;
    // let sw_int = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    // esp_rtos::start(timg0.timer0, sw_int.software_interrupt0);

    // create the modulator
    // FIXME I2S rust support is garbage
    // let dma_channel = peripherals.DMA_I2S0;
    // let (mut rx_buffer, rx_descriptors, _, _) = esp_hal::dma_buffers!(4 * 4092, 0);
    // use esp_hal::i2s::master::I2s;
    // use esp_hal::i2s::master::Config;
    // use esp_hal::time::Rate;
    // use esp_hal::i2s::master::DataFormat;
    // let i2s = I2s::new(
    //     peripherals.I2S0,
    //     dma_channel,
    //     Config::new_tdm_pcm_short()
    //         // Input configuration
    //         .with_sample_rate(Rate::from_hz(PCM_SR_HZ))
    //         .with_data_format(DataFormat::Data16Channel16)
    //         .with_channels(Channels::RIGHT)
    //         // Output configuration
    // ).unwrap();
    // let i2s = i2s.with_mclk(peripherals.GPIO0);
    // let mut i2s_rx = i2s
    //     .i2s_rx
    //     .with_bclk(peripherals.GPIO1)
    //     .with_ws(peripherals.GPIO2)
    //     .with_din(peripherals.GPIO5)
    //     .build(rx_descriptors);
    // let mut transfer = i2s_rx.read_dma_circular(&mut rx_buffer).unwrap();

    // initialize the bluetooth hardware
    // use default 64K heap (required by radio)
    // create_heap!();
    // FIXME esp32_radio currently only supports BLE
    // https://github.com/esp-rs/esp-hal/issues/3401

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