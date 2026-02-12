#![no_std]
#![no_main]

// provide the shared crates via re-export
use common::*;
use soc_esp32::*;   // provides the panic handler


#[soc_esp32::esp_rtos::main]
// async fn main(spawner: soc_esp32::embassy_executor::Spawner) -> ! {
async fn main(_spawner: embassy_executor::Spawner) -> ! {
    // TODO implement (work in diy-esp32)

    loop {
    }
}
