use crate::packets::{
    IdentityInformation, PatchError, PatchFile, PatchRequest, PatchResponse, PatchResult,
};
use color_eyre::eyre::eyre;
use color_eyre::Result;
use skrillax_stream::handshake::PassiveSecuritySetup;
use skrillax_stream::stream::SilkroadTcpExt;
use std::net::ToSocketAddrs;
use std::time::Duration;
use tokio::net::TcpSocket;
use tokio::sync::mpsc::{channel, Receiver};

const JOYMAX_GATEWAY_ADDRESS: &str = "gwgt1.joymax.com:15779";

pub struct UpdateInformation {
    pub new_version: u32,
    pub http_server: String,
    pub files: Vec<PatchFile>,
}

pub fn start_checker(start_version: u32, pause: Duration) -> Receiver<UpdateInformation> {
    let (sender, receiver) = channel(5);
    tokio::spawn(async move {
        let mut current_version = start_version;
        loop {
            match fetch_patch_files(current_version).await {
                Ok(None) => {}
                Ok(Some(patch)) => {
                    current_version = patch.new_version;
                    if let Err(e) = sender.send(patch).await {
                        break;
                    }
                }
                Err(e) => {
                    break;
                }
            }
            tokio::time::sleep(pause).await;
        }
    });
    receiver
}

async fn fetch_patch_files(my_version: u32) -> Result<Option<UpdateInformation>> {
    let domain = JOYMAX_GATEWAY_ADDRESS.to_socket_addrs()?.next().unwrap();
    let socket = TcpSocket::new_v4()?;
    let connection = socket.connect(domain).await?;
    let (mut reader, mut writer) = connection.into_silkroad_stream();
    PassiveSecuritySetup::handle(&mut reader, &mut writer).await?;
    writer
        .send(IdentityInformation {
            module_name: "SR_Client".to_owned(),
            locality: 0,
        })
        .await?;
    let _ = reader.next_packet::<IdentityInformation>().await?;
    writer
        .send(PatchRequest {
            content: 0x12,
            module: "SR_Client".to_string(),
            version: my_version,
        })
        .await?;
    let response = reader.next_packet::<PatchResponse>().await?;
    match response.result {
        PatchResult::UpToDate { .. } => Ok(None),
        PatchResult::Problem { error } => match error {
            PatchError::InvalidVersion => Err(eyre!("Invalid version presented.")),
            PatchError::Update {
                current_version,
                http_server,
                patch_files,
                ..
            } => {
                println!("We should update to v{}", current_version);
                Ok(Some(UpdateInformation {
                    new_version: current_version,
                    http_server,
                    files: patch_files,
                }))
            }
            PatchError::Offline => Err(eyre!("Offline.")),
            PatchError::InvalidClient => Err(eyre!("Invalid client?!.")),
            PatchError::PatchDisabled => {
                Err(eyre!("Patch no longer allowed, needs a fresh install."))
            }
        },
    }
}
