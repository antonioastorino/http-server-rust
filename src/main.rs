use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
mod http_handler;
mod http_response_status;
use http_handler::request::*;
use http_response_status::*;

fn main() {
    println!("Hello, TCP!");
    let listener: TcpListener = TcpListener::bind("127.0.0.1:8081").unwrap();
    for incoming in listener.incoming() {
        let mut stream = incoming.unwrap();
        let request_data: RequestData = handle_connection(&stream);
        /*
        let mut response_header: String;
        if request_data.http_version == RequestHttpVersion::Unknown {
                response_header_string = http_handler::response_header::create(ResponseStatus::HttpVersionNotSupported)
            }
        else if response_header.method == RequestData::Unknown {
            http_handler::response_header::create(ResponseStatus::MethodNotAllowed);
        }
        else if response_header.address_type == RequestAddressType::Unknonw {
            http_handler::response_header::create(ResponseStatus::NotFound);
        }
        */

        println!("{:?}", &request_data);
        //        stream.write(response_header.as_bytes()).unwrap();
        //        stream.flush().unwrap();
    }
}

fn handle_connection(mut stream: &TcpStream) -> RequestData {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let text = String::from_utf8_lossy(&buffer[..]);
    return RequestData::new(text.to_string());
}
