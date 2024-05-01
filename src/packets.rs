use skrillax_packet::Packet;
use skrillax_serde::{ByteSize, Deserialize, Serialize};

#[derive(Packet, ByteSize, Serialize, Deserialize)]
#[packet(opcode = 0x2001)]
pub struct IdentityInformation {
    pub module_name: String,
    pub locality: u8,
}

#[derive(Packet, Serialize, Deserialize, ByteSize)]
#[packet(opcode = 0x6100)]
pub struct PatchRequest {
    pub content: u8,
    pub module: String,
    pub version: u32,
}

#[derive(Deserialize, Serialize, ByteSize)]
pub enum PatchResult {
    #[silkroad(value = 1)]
    UpToDate { unknown: u8 },
    #[silkroad(value = 2)]
    Problem { error: PatchError },
}

#[derive(Deserialize, Serialize, ByteSize)]
pub enum PatchError {
    #[silkroad(value = 1)]
    InvalidVersion,
    #[silkroad(value = 2)]
    Update {
        server_ip: String,
        server_port: u16,
        current_version: u32,
        #[silkroad(list_type = "has-more")]
        patch_files: Vec<PatchFile>,
        http_server: String,
    },
    #[silkroad(value = 3)]
    Offline,
    #[silkroad(value = 4)]
    InvalidClient,
    #[silkroad(value = 5)]
    PatchDisabled,
}

#[derive(Deserialize, Serialize, ByteSize)]
pub struct PatchFile {
    pub file_id: u32,
    pub filename: String,
    pub file_path: String,
    pub size: u32,
    pub in_pk2: bool,
}

#[derive(Packet, Deserialize, Serialize, ByteSize)]
#[packet(opcode = 0xA100, massive = true)]
pub struct PatchResponse {
    pub result: PatchResult,
}
