#![cfg_attr(not(feature = "std"), no_std)]


mod companion;
pub use self::companion::Companion;
pub use self::companion::CompanionBle;