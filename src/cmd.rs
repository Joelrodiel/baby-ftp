use crate::ftp::*;
use net2::TcpBuilder;
use std::io::{self, Write};

pub fn connect(mut _info: &mut ConnectionInfo, mut _client: &mut ClientInfo) -> bool {
    let _status = client_connect(&mut _client);
    let mut _stream = TcpBuilder::new_v4().unwrap()
        .to_tcp_stream().unwrap();
    match _status {
        Ok(s) => {
            _stream = s;
            _info.is_connected = true;
            _info.is_closing = false;
        },
        Err(e) => {
            println!("Error: {}", e);
            return false
        }
    }

    print_reply(&_stream);

    let mut name: String = "".to_string();
    let mut pass: String = "".to_string();

    print!("Name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).unwrap();

    send_reply(&mut _stream, "USER", &(name.replace("\n", "")));

    print_reply(&_stream);

    print!("Pass: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut pass).unwrap();

    send_reply(&mut _stream, "PASS", &(pass.replace("\n", "")));
    pass.clear();

    let login_code = print_reply(&_stream);

    if login_code != "230" {
        _info.is_closing = true;
    }

    _client.username = name;
    return true
}
