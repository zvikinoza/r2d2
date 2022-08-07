#![allow(non_snake_case)]

mod master;
pub mod runner;
mod worker;

use clap::Parser;

pub const RUNNER_ADDR: &str = "[::1]:51116";
pub const MASTER_ADDR: &str = "[::1]:51117";

#[derive(Parser, Debug)]
#[clap(about = "Stiffler")]
pub struct Args {
    #[clap(
        short,
        long,
        parse(from_os_str),
        default_value = "./single_worker/input"
    )]
    pub intput_path: std::path::PathBuf,

    #[clap(
        short,
        long,
        parse(from_os_str),
        default_value = "./single_worker/output"
    )]
    pub output_path: std::path::PathBuf,

    #[clap(long, takes_value = false)]
    master: bool,
}

pub async fn initialize() {
    let args = Args::parse();
    if args.master {
        master::start().await.expect("Failed to start master");
        std::process::exit(0);
    } else {
        worker::start().await.expect("Failed to start worker");
    }
    // drop(args);
}

/// terminate is only reachable for worker nodes
pub async fn terminate() {
    match worker::task_finished().await {
        _ => (),
    }
}
