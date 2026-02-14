pub struct Companion {}

impl Companion {
    pub fn new() -> Self {
        Companion {}
    }

    /// https://github.com/meshcore-dev/MeshCore/tree/main/examples/companion_radio
    pub fn run() -> ! {
        loop {
            // the_mesh.loop();
            // {
            //   BaseChatMesh::loop();
            //   Mesh::loop();
            //  Dispatcher::loop();
            //   if (txt_send_timeout && millisHasNowPassed(txt_send_timeout)) {
            //     // failed to get an ACK
            //     onSendTimeout();
            //     txt_send_timeout = 0;
            //   }

            //   if (_pendingLoopback) {
            //     onRecvPacket(_pendingLoopback);  // loop-back, as if received over radio
            //     releasePacket(_pendingLoopback);   // undo the obtainNewPacket()
            //     _pendingLoopback = NULL;
            //   }

            //   if (_cli_rescue) {
            //     checkCLIRescueCmd();
            //   } else {
            //     checkSerialInterface();
            //   }

            //   // is there are pending dirty contacts write needed?
            //   if (dirty_contacts_expiry && millisHasNowPassed(dirty_contacts_expiry)) {
            //     saveContacts();
            //     dirty_contacts_expiry = 0;
            //   }

            // #ifdef DISPLAY_CLASS
            //   if (_ui) _ui->setHasConnection(_serial->isConnected());
            // #endif
            // }
            // sensors.loop();
            // #ifdef DISPLAY_CLASS
            //     ui_task.loop();
            // #endif
            // rtc_clock.tick();
        }
    }
}

//------------------------------------------------------------------------------
// BLE Support
pub struct CompanionBle {}
const BLE_MTU_MAX: usize = 1024;

// GATT Server definition
use trouble_host::prelude::*;
#[gatt_server(connections_max = 1)]
struct Server {
    meschore: MeshCoreService,
}

/// BLE Host per https://github.com/meshcore-dev/MeshCore/blob/main/docs/companion_protocol.md
#[gatt_service(uuid = "6E400001-B5A3-F393-E0A9-E50E24DCCA9E")]
struct MeshCoreService {
    // support for serial interface
    //------------------------------------------------------------------------------
    // Rx (App -> Firmware)
    #[characteristic(uuid = "6E400002-B5A3-F393-E0A9-E50E24DCCA9E", write, notify)]
    pub rx: heapless::Vec<u8, BLE_MTU_MAX>,

    // Tx (Firmware -> App)
    #[characteristic(uuid = "6E400003-B5A3-F393-E0A9-E50E24DCCA9E", read, notify)]
    pub tx: heapless::Vec<u8, BLE_MTU_MAX>,
    //------------------------------------------------------------------------------
    // TODO extend to provide true BLE interface
}
