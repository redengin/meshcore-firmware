#![cfg_attr(not(feature = "std"), no_std)]

// firmware roles (i.e. companion, repeater, room)
mod companion;
pub use self::companion::Companion;
mod repeater;
pub use self::repeater::Repeater;
// pub use self::room::Room;

// application interfaces (BLE, USB Serial, etc.)
pub use trouble_host;   // BLE host library
pub mod app_interface;

// pub trait MeshCoreFirmareLora {
//     // send LoRa data per MeshCore standards
//     fn lora_tx(bytes: &[u8]);

//     // receive LoRa data
//     fn lora_rx() -> [u8];
// }

// /// support MeshCore API
// pub trait MeshCoreFirmareApi {
//     fn send_channel();
//     fn get_contacts() -> ( /* FIXME */);
//     /*
//     ...
//     */
// }