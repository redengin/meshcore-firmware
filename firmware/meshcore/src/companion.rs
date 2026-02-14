pub struct Companion
{
}

impl Companion {
    pub fn new() -> Self
    {
        Companion{}
    }

    /// https://github.com/meshcore-dev/MeshCore/tree/main/examples/companion_radio
    pub fn run() -> !
    {
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

/// BLE Host per https://github.com/meshcore-dev/MeshCore/blob/main/docs/companion_protocol.md
pub struct CompanionBle {

}
