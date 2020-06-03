use std::env;
use std::io::{self, Write};
use std::net::{Shutdown};

mod cmd;
mod ftp;

use cmd::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut server_info = ftp::ConnectionInfo::default();
    let mut client_info = ftp::ClientInfo::default();

    if args.len() > 1 {
        cmd_connect(args, &mut server_info, &mut client_info);
    }

    loop {
        if server_info.is_closing { break; }
        print!("ftp> ");
        io::stdout().flush().unwrap();
        let mut cmd = "".to_string();
        io::stdin().read_line(&mut cmd).unwrap();

        let cmd_args: Vec<String> = cmd.split_whitespace().map(String::from).collect();

        match cmd_args.get(0).unwrap().as_str() {
            "connect" => {
                if cmd_args.len() > 1 {
                    cmd_connect(cmd_args, &mut server_info, &mut client_info);
                } else {
                    println!("Usage: connect host-name [port]");
                }
            },
            "disconnect" | "exit" => {
                break;
            },
            "status" => {
                // cmd_status(server_info, client_info);
            }
            _ => {
                println!("Invalid command.");
            }
        }
    }

    if server_info.is_connected {
        server_info.data_conc.shutdown(Shutdown::Both).unwrap();
        println!("221 Goodbye");
    }
}

