#![cfg_attr(not(feature = "std"), no_std)]

// firmware roles (i.e. companion, repeater, room)
mod companion;
pub use self::companion::Companion;
// pub use self::repeater::Repeater;
// pub use self::room::Room;

// application interfaces (BLE, USB Serial, etc.)
mod interface;
pub use self::interface::CompanionBle;
// pub use self::interface::CompanionSerial;
// pub use self::interface::CompanionWifi;