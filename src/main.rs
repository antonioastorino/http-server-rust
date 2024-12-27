mod http_handler;
use http_handler::common::ContentType;
use http_handler::request::*;
use http_handler::response::*;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::os::fd::AsRawFd;
extern crate core;
const BUFF_READ_SIZE: usize = 32768;

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
            payload.content_length,
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
        loop {
            count = reader.read_line(&mut buffer).unwrap();
            // read_line() includes the EOL -> len() == 2 means "\r\n" only, i.e., empty line
            if count <= 2 {
                // End of header detected
                break;
            }
        }
        println!(
            "---- request header start ----\n{}---- request header end ----",
            buffer
        );
        let request_data: RequestHeader = RequestHeader::new(&buffer);
        if request_data.payload.content_length > 0 {
            let mut capacity: usize = request_data.payload.content_length.try_into().unwrap();
            println!(
                "Received payload of type {:?}",
                request_data.payload.content_type
            );
            let mut bytes_read: usize;
            let mut body: [u8; BUFF_READ_SIZE] = [0; BUFF_READ_SIZE];
            let mut out_file = std::fs::File::options()
                .write(true)
                .create(true)
                .truncate(true)
                .open(format!("artifacts/{:?}-data", request_data.payload.content_type).as_str())
                .unwrap();
            loop {
                bytes_read = reader.read(&mut body).unwrap();
                println!("capacity left {}, bytes read {}", capacity, bytes_read);
                if request_data.payload.content_type == ContentType::Png {
                    out_file.write_all(&body[0..bytes_read]).unwrap();
                } else {
                    write!(
                        &mut out_file,
                        "{}",
                        std::str::from_utf8(&body[0..bytes_read]).unwrap()
                    )
                    .unwrap();
                }
                if capacity > bytes_read {
                    capacity -= bytes_read;
                } else {
                    if capacity != bytes_read {
                        panic!("File received larger than expected! This should not happen");
                    }
                    break;
                }
            }
        }
        let response_data: Response = Response::new(&request_data);
        let mut response_header: String = format!(
            "{} {}\r\nContent-Length: {}\r\n",
            request_data.http_version.to_str(),
            response_data.status.to_str(),
            response_data.payload.content_length,
        )
        .to_string();

        if response_data.payload.content_length > 0 {
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
