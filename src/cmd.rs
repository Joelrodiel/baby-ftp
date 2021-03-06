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
                print!("Active");
            }
            FTPModes::Passive => {
                print!("Passive");
            }
            FTPModes::Both => {
                print!("Both");
            }
        }
        print!("\nData type: ");
        match &_info.data_type {
            FTPTypes::ASCII => {
                println!("ASCII");
            }
            FTPTypes::BINARY => {
                println!("BINARY");
            }
        }
    } else {
        println!("Not connected.");
    }
}

pub fn cmd_set_type(mut _info: &mut ConnectionInfo, typ: FTPTypes) {
    let type_code = match &typ {
        FTPTypes::ASCII => {
            "A"
        }
        FTPTypes::BINARY => {
            "I"
        }
    };
    _info.data_type = typ;
    send_reply(&mut _info.data_conc, "TYPE", type_code);
    print_reply(&_info.data_conc);
}

pub fn cmd_get_system(mut _info: &mut ConnectionInfo) {
    send_reply(&mut _info.data_conc, "SYST", "");
    print_reply(&_info.data_conc);
}
