use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
mod http_handler;
use http_handler::request::*;
use http_handler::response::*;
use std::os::fd::AsRawFd;
extern crate core;

extern "C" {
    fn tcp_utils_send_file(
        file_path: *const std::os::raw::c_char,
        file_size: u64,
        socket: i32,
    ) -> i32;
}

fn main() {
    println!("Hello, TCP!");
    let listener: TcpListener = TcpListener::bind("127.0.0.1:8081").unwrap();
    for incoming in listener.incoming() {
        let mut stream = incoming.unwrap();
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let text = String::from_utf8_lossy(&buffer[..]);
        let request_data: Request = Request::new(text.to_string());
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
            stream.write(response_header.as_bytes()).unwrap();
            let c_string = std::ffi::CString::new(response_data.payload.path).unwrap();
            println!("File path {:?}", &c_string);
            println!("File size {}", response_data.payload.content_size);
            println!("Socket {}", stream.as_raw_fd());
            unsafe {
                tcp_utils_send_file(
                    (&c_string).as_ptr(),
                    response_data.payload.content_size,
                    stream.as_raw_fd(),
                );
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
