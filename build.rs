extern crate cc;
use std::io::Read;
use std::io::Write;

fn main() {
    {
        let mut template_content = String::new();
        let mut tuples_content = String::new();
        let mut src = std::fs::File::open("assets/template_files.txt").unwrap();
        src.read_to_string(&mut template_content).unwrap();
        drop(src);
        let mut src = std::fs::File::open("tuples.rs").unwrap();
        src.read_to_string(&mut tuples_content).unwrap();
        drop(src);

        let mut new_content =
            template_content.replace("%valid_addresses%", tuples_content.as_str());
        new_content = format!(
            "{}\n{}\n{}\n{}\n{}\n\n{}",
            "/*********************************************",
            " *   THIS FILE IS GENERATED WHEN BUILDING    *",
            " *  modify 'tuples.rs' and run               *",
            " * `cargo clean && cargo build` to update it *",
            " *********************************************/",
            new_content
        );
        let mut f = std::fs::File::options()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open("src/http_handler/files.rs")
            .unwrap();
        writeln!(&mut f, "{}", new_content).unwrap();
        drop(f);
        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg("rustfmt src/http_handler/files.rs")
            .output()
            .expect("failed");
        let hello = output.stderr;
        assert!(hello.len() == 0);
    }

    cc::Build::new()
        .file("c/sendfile.c")
        .compile("tcp_utils_send_file");
}
