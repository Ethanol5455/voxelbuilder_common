#[allow(dead_code)]
pub enum PacketType {
    PlayerConnect,
    PlayerDisconnect,
    PlayerInfoRequest, // Get saved player data from file (if available)
    PlayerInfoData,    // Data about a player to save, sent at a fixed interval from the client
    ChunkRequest,      // Request from the client to send data about a chunk
    ChunkUpdate,       // Request from the client to update a chunk
    ChunkContents,     // The contents of a chunk as requested by the client
                       // TODO: Add server message to client // Send a message from the server to the client
                       // TODO: Add client command to server // Send a command from the client to the server
}

pub enum ChunkUpdateType {
    PlaceBlockEvent,
    DestroyBlockEvent,
}
