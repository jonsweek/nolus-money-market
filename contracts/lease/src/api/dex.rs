use serde::{Deserialize, Serialize};

use sdk::schemars::{self, JsonSchema};

/// Parameters needed to operate with the Dex network
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct ConnectionParams {
    /// The IBC connection to the DEX used to transfer in/out, swap and hold assets
    pub connection_id: String,
    /// The IBC ICS-20 channel used to transfer assets in/out.
    /// It must be established over the same connection.
    pub transfer_channel: Ics20Channel,
}

/// IBC ICS-20 channel parameters
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Ics20Channel {
    /// The id of the local endpoint
    pub local_endpoint: String,
    /// The id of the remote endpoint
    pub remote_endpoint: String,
}
