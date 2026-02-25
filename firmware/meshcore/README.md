MeshCore Implementation
================================================================================
* LoRa communication per MeshCore
    * implements MeshCore design to minimize congestion

Application Interface
--------------------------------------------------------------------------------
As the MeshCore design evolves, the protocols for both LoRa communications and
application interfaces to nodes (companion, repeater, room) are being unified.

Where this implementation focuses on the needs of firwmare nodes, the
implementation also supports use by client software.

<!-- Do we extend MeshCore or fork....

Companion Extension
--------------------------------------------------------------------------------
### Multiple User Support
Rather than dedicate a node to a specific user, any authorized user should be
able to use a `companion node`.

Authorization is managed by the connection to the node.
* BLE  - requires a shared password (pin-code)
* WiFi - router may require a password

The `companion node` will manage the bufferred communications of users -
prioritized toward active users.

### Bot - monitor a channel, and provide responses upon `triggers`
Provide the ability to update the `bot` independenatly - i.e. don't require a
full firmware update.


Room Extension
--------------------------------------------------------------------------------
### Bot - monitor a channel, and provide responses upon `triggers`
Provide the ability to update the `bot` independenatly - i.e. don't require a
full firmware update.

-->




