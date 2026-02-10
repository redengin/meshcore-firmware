Meshcore Firmware implemented in Rust
================================================================================

[Meshcore](https://meshcore.co.uk/) - 
    We connect people and things, without using the internet

Meshcore provides public forums on the mesh, where [Meshtastic](https://meshtastic.org/) 
is designed for private user-to-user conversations.

Meshcore is much more like the old days of [BBS](https://en.wikipedia.org/wiki/Bulletin_board_system) - low bandwidth access to public communication.
Meshcore `room servers` allow nodes to perform like a BBS.

## Adapting to the masses
Neither Meshcore nor Meshtastic completely satisfy users.

Where the hardware permits, all mesh protocols will be supported.
* bridges between mesh protocols increase access to users


Features
================================================================================
* node binding to WiFi - like Meshtastic, provide access to the node on the
    local WiFi LAN.
    * This allows a WiFi AccessPoint to provide access to the mesh
* extended configuration - provide additional configuration parameters per the
    node instance.
* support for novel `room server` nodes
    * allow owner to add `room server` agent to their node
    * support distributed `room service` by coordinating `room server` nodes


