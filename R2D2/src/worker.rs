use r2d2::r2d2_client::R2d2Client;
use r2d2::{ReadyRequest, TaskFinishedRequest};

use crate::MASTER_ADDR;

pub mod r2d2 {
    tonic::include_proto!("r2d2");
}

pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = R2d2Client::connect(MASTER_ADDR).await?;

    let request = tonic::Request::new(ReadyRequest {});
    let response = client.ready(request).await?;
    if !response.into_inner().start {
        unimplemented!();
    }

    Ok(())
}

pub async fn task_finished() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = R2d2Client::connect(MASTER_ADDR).await?;

    let request = tonic::Request::new(TaskFinishedRequest {});
    let _response = client.task_finished(request).await?;

    Ok(())
}

#[allow(unused)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    start().await
}