use std::env;
use std::net::{TcpStream, ToSocketAddrs};
use std::io::{self, Write, BufReader, BufRead};
use regex::Regex;
use net2::TcpBuilder;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut server_info = ConnectionInfo::default();
    let mut client_info = ClientInfo::default();

    if args.len() > 1 {
        client_info.server_name = String::from(args.get(1).unwrap());

        if !client_info.server_name.contains(":") {
            if args.len() == 2 {
                client_info.server_name.push_str(":21");
            } else {
                client_info.server_name.push_str(":");
                client_info.server_name.push_str(args.get(2).unwrap());
            }
        }

        cmd_connect(&mut server_info, &mut client_info);
    }

    loop {
        if server_info.is_closing { break; }
        print!("ftp> ");
        io::stdout().flush().unwrap();
        let mut cmd = "".to_string();
        io::stdin().read_line(&mut cmd).unwrap();

        let cmd_args: Vec<&str> = cmd.split_whitespace().map(|x| x).collect();

        match cmd_args.get(0).unwrap() {
            &"connect" => {
                if cmd_args.len() > 1 {
                    client_info.server_name = cmd_args.get(1).unwrap().to_string();

                    cmd_connect(&mut server_info, &mut client_info);
                } else {
                    println!("Usage: connect host-name [port]");
                }
            },
            _ => {

            }
        }
    }
}

fn cmd_connect(mut _info: &mut ConnectionInfo, mut _client: &mut ClientInfo) {
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
            std::process::exit(65);
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
}

fn client_connect(_info: &mut ClientInfo) -> Result<TcpStream, &'static str> {
    let mut _address_iter = match _info.server_name.as_str().to_socket_addrs() {
        Ok(_iter) => {
            _iter
        }
        Err(_) => {
            return Err("Invalid socket address")
        }
    };

    let mut _stream = TcpBuilder::new_v4().unwrap().to_tcp_stream().unwrap();
    let mut _address_iter_val = _address_iter.next();
    let mut _address = _address_iter_val.unwrap().to_string();
    let mut _stream_iter = TcpStream::connect(_address_iter_val.unwrap());

    while _address_iter_val.is_some() && _stream_iter.is_err() {
        _address = _address_iter_val.unwrap().to_string();
        _stream_iter = TcpStream::connect(_address_iter_val.unwrap());
        _address_iter_val = _address_iter.next();
    }

    if _stream_iter.is_err() {
        Err("Could not connect!")
    } else {
        println!("Connected to {}", _address);
        _stream = _stream_iter.unwrap();
        Ok(_stream)
    }
}

// TODO: Color replies by protocol code.
fn print_reply(_stream: &TcpStream) -> String {
    let mut reader = BufReader::new(_stream);
    let mut _recieved = "".to_string();
    let mut code = "".to_string();
    let code_re = Regex::new("(\\d{3})").unwrap();
    let end_re = Regex::new("^(\\d+[ ])|\\n(\\d+[ ])").unwrap();

    while _recieved == ""  || !end_re.is_match(&_recieved.as_str()) {
        reader.read_line(&mut _recieved).unwrap();
        if code == "" && code_re.is_match(&_recieved.as_str()) {
            let cap = code_re.captures(&_recieved).unwrap();
            code = cap[1].to_string();
        }
    }

    print!("{}", _recieved);
    code
}

fn send_reply(mut _stream: &mut TcpStream, _name: &str, _info: &str) {
    _stream.write(generate_msg(_name, _info).as_bytes()).unwrap();
}

fn generate_msg(_name: &str, _info: &str) -> String {
    return String::from(_name.to_string() + " " + _info + "\r\n")
}

pub enum FTPModes { Active, Passive, Both }
impl Default for FTPModes {
    fn default() -> Self {
        FTPModes::Both
    }
}

pub enum FTPTypes { ASCII, BINARY }
impl Default for FTPTypes {
    fn default() -> Self {
        FTPTypes::ASCII
    }
}

pub struct ConnectionInfo {
    pub connect_mode: FTPModes,
    pub data_conc: TcpStream,
    pub data_type: FTPTypes,
    pub is_connected: bool,
    pub is_data_up: bool,
    pub is_closing: bool,
}

impl Default for ConnectionInfo {
    fn default() -> Self {
        ConnectionInfo {
            connect_mode: FTPModes::Active,
            data_conc: TcpBuilder::new_v4().unwrap()
                .to_tcp_stream().unwrap(),
            data_type: FTPTypes::ASCII,
            is_connected: bool::default(),
            is_data_up: bool::default(),
            is_closing: bool::default(),
        }
    }
}

pub struct ClientInfo {
    pub server_name: String,
    pub connect_mode: FTPModes,
    pub username: String,
}
impl Default for ClientInfo {
    fn default() -> Self {
        ClientInfo {
            server_name: "localhost:21".to_string(),
            connect_mode: FTPModes::Passive,
            username: "root".to_string(),
        }
    }
}
