MeshCore Implementation
================================================================================
* LoRa communication using MeshCore packets
    * per to MeshCore's architecture designed to minimize congestion
* BLE GATT service
    * support the standard app
    * provide additional GATT interfaces (see [MeshCore Role Support](#meshcore-role-support))
* WiFi service
    * support the standard app
    * provide additional interfaces (see [MeshCore Role Support](#meshcore-role-support))

MeshCore Role Support
================================================================================
MeshCore architecture defines roles as:
* router - repeats packets to extend reach
* companion - communicates with the mesh as a user
* room - provides buffer of communications

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





