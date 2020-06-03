use std::net::{TcpStream, ToSocketAddrs};
use std::io::{self, Write, BufReader, BufRead};
use std::time::Duration;
use regex::Regex;
use net2::TcpBuilder;

pub fn connect(mut _info: &mut ConnectionInfo, mut _client: &mut ClientInfo) -> bool {
    let _status = conn_tcp_stream(&mut _client);
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
    _info.data_conc = _stream;
    return true
}

pub fn conn_tcp_stream(_info: &mut ClientInfo) -> Result<TcpStream, &'static str> {
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
    if let Ok(stream) = TcpStream::connect_timeout(&_address_iter_val.unwrap(), Duration::new(5, 0)) {
        return Ok(stream)
    }
    
    Err("Could not connect!")
}

// TODO: Color replies by protocol code.
pub fn print_reply(_stream: &TcpStream) -> String {
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

pub fn send_reply(mut _stream: &mut TcpStream, _name: &str, _info: &str) {
    _stream.write(generate_msg(_name, _info).as_bytes()).unwrap();
}

pub fn generate_msg(_name: &str, _info: &str) -> String {
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
