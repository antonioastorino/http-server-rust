mod http_handler;
use http_handler::request::*;
use http_handler::response::*;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::os::fd::AsRawFd;
extern crate core;

fn sendfile(payload: &ResponsePayload, stream: &mut TcpStream) -> i32 {
    extern "C" {
        fn tcp_utils_send_file(
            file_path: *const std::os::raw::c_char,
            file_size: u64,
            socket: i32,
        ) -> i32;
    }
    let c_string = std::ffi::CString::new(payload.path).unwrap();
    unsafe {
        return tcp_utils_send_file(
            (&c_string).as_ptr(),
            payload.content_size,
            stream.as_raw_fd(),
        );
    }
}

fn main() {
    println!("Hello, TCP!");
    let listener: TcpListener = TcpListener::bind("127.0.0.1:8081").unwrap();
    for incoming in listener.incoming() {
        let mut stream = incoming.unwrap();
        let mut reader = std::io::BufReader::new(stream.try_clone().unwrap());
        let mut buffer = String::new();
        let mut count: usize;
        println!("{}", buffer);
        loop {
            count = reader.read_line(&mut buffer).unwrap();
            println!("{}", count);
            if count <= 2 {
                break;
            }
        }
        println!(
            "---- request header start ----\n{}---- request header end ----",
            buffer
        );

        let request_data: RequestHeader = RequestHeader::new(buffer);
        let response_data: Response = Response::new(&request_data);
        let mut response_header: String = format!(
            "{} {}\r\nContent-Length: {}\r\n",
            request_data.http_version.to_str(),
            response_data.status.to_str(),
            response_data.payload.content_size,
        )
        .to_string();

        if response_data.payload.content_size > 0 {
            response_header.push_str("Content-Type: ");
            response_header.push_str(response_data.payload.content_type.to_str());
            response_header.push_str("\r\n\r\n");
            println!("{}", response_header);
            stream.write(response_header.as_bytes()).unwrap();
            if sendfile(&response_data.payload, &mut stream) < 0 {
                println!("Error while sending {:?}", response_data.payload);
            }
            stream.flush().unwrap();
        } else {
            response_header.push_str("\r\n");
            println!("{}", response_header);
            stream.write(response_header.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}
