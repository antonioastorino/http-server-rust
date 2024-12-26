extern crate cc;

fn main() {
    cc::Build::new()
        .file("c/sendfile.c")
        .compile("tcp_utils_send_file");
}
