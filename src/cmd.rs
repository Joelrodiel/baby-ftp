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

// pub fn disconnect(mut _info: &mut ConnectionInfo, mut _stream: )
