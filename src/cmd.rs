use crate::ftp::*;

pub fn cmd_connect(args: Vec<String>, mut _info: &mut ConnectionInfo, mut _client: &mut ClientInfo) {
    _client.server_name = String::from(args.get(1).unwrap());

    if !_client.server_name.contains(":") {
        if args.len() == 2 {
            _client.server_name.push_str(":21");
        } else {
            _client.server_name.push_str(":");
            _client.server_name.push_str(args.get(2).unwrap());
        }
    }

    connect(&mut _info, &mut _client);
}

pub fn cmd_status(_info: &ConnectionInfo, _client: &ClientInfo) {
    if _info.is_connected {
        print!("Connected to {}\nUser: {}\nMode: ", _client.server_name, _client.username);
        match &_info.connect_mode {
            FTPModes::Active => {
                println!("Active");
            }
            FTPModes::Passive => {
                println!("Passive");
            }
            FTPModes::Both => {
                println!("Both");
            }
        }
    } else {
        println!("Not connected.");
    }
}
