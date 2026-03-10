

pub struct Repeater {}


impl Repeater {

    /**
     * Manage the LoRa as a MeshCore repeater
     * 
     */
    pub async fn run(&self) -> ! {
        loop {
            // TODO listen for packet(s)

            // TODO process each packet - retransmit per configuration

        }
    }
}