use clap::Parser;
use std::fs;
use std::net::TcpStream;

#[derive(Parser, Debug)]
#[clap(about="Stiffler")]
struct Args {
    #[clap(short, long, parse(from_os_str), default_value = "./master_w_single_worker/input")]
    intput_path: std::path::PathBuf,

    #[clap(short, long, parse(from_os_str), default_value = "./master_w_single_worker/output")]
    output_path: std::path::PathBuf,

    #[clap(long, takes_value=false)]
    master: bool,
}

pub mod blast {
    use std::{net::TcpListener, thread};
    use std::io::{Read, Write};

    use super::*;
    pub fn init() {
        {
            let args = Args::parse();
            if args.master {
                let listener = TcpListener::bind(format!("127.0.0.1:{}", runner::MASTER_PORT))
                    .expect("failed to bind master port");

                println!("master listening on");
                for stream in listener.incoming() {
                    match stream {
                        Ok(stream) => {
                            println!("master got new connection: {}", stream.peer_addr().unwrap());
                            // thread::spawn(move|| {
                                // connection succeeded
                                handle_worker(stream)
                            // });
                        }
                        Err(e) => {
                            println!("Error: {}", e);
                        }
                    }
                    break;
                }
                
                // update runner
                match TcpStream::connect(format!("127.0.0.1:{}", runner::RUNNER_PORT)) {
                    Ok(mut stream) => {
                        stream.write(b"Done").unwrap();
                    }
                    Err(e) => {
                        println!("Errr connecting to runner: {}", e);
                    }
                }

                // close the socket server
                // drop(listener);

            } else {
                match TcpStream::connect(format!("127.0.0.1:{}", runner::MASTER_PORT)) {
                    Ok(mut stream) => {
                        println!("worker uccessfully connected to master in port {}", runner::MASTER_PORT);

                        stream.write(b"Can I start?").unwrap();
                        println!("Sent `Can I start?`, awaiting reply...");

                        let mut data = [0 as u8; 1024]; // 
                        let size = stream.read(&mut data)
                            .expect(&format!("Failed to receive response from master on Question: Can I start?"));
                        if &data[..size] == b"You must!" {
                            return;
                        } else {
                            println!("exiting. Received: {:?}", &data[..size]);
                        }
                    }, 
                    Err(e) => {
                        println!("failded to connect Error: {}", e);
                    },
                }
            }
        } // drop(args);
        std::process::exit(0);

    }
    
    fn handle_worker(mut stream: TcpStream) {
        let mut data = [0 as u8; 1024]; // using 1024 byte buffer
        let size = stream.read(&mut data)
            .expect(&format!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap()));
        if &data[..size] == b"Can I start?" {
            println!("worker asked: Can I start?");
            stream.write(b"You must!").unwrap();
        } else {
            println!("not expected, worker asked: {:?}", &data[..size]);
            return;
        }
        thread::sleep(std::time::Duration::from_millis(2000));
        let size = stream.read(&mut data)
            .expect(&format!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap()));
        if &data[..size] == b"Task finished" {
            println!("worker responded with: Task finished");
        } else {
            println!("unexpected, worker responded with: {:?}", &data[..size]);
        }
    }

    /// terminate is only reachable for worker nodes
    pub fn terminate() {
        match TcpStream::connect(format!("127.0.0.1:{}", runner::MASTER_PORT)) {
            Ok(mut stream) => {
                println!(" termination worker Successfully connected to master in port {}", runner::MASTER_PORT);
                stream.write(b"Task finished").unwrap();
                println!("Sent `task finsihed`, terminateing");
            }, 
            Err(e) => {
                println!("failded to connect & update master about termiantion and task finish : {}", e);
            },
        }
    }
}

fn main() {
    blast::init();
    let args = Args::parse();

    let input = fs::read_to_string(args.intput_path).unwrap();
    fs::write(args.output_path, input).unwrap();

    blast::terminate();
}