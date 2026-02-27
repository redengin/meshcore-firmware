
/// BLE host to support App
pub mod ble;
// pub use self::ble::CompanionBle;

// FIXME not necessary to expose
pub mod companion_protocol;

mod command_stream_handler;
pub use self::command_stream_handler::CommandStreamHandler;