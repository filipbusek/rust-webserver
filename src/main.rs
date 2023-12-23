use std::fs;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use rand::Rng;

mod workers;
use workers::ThreadPool;

mod config;
use config::Config;

fn handle_connection(mut stream: TcpStream, docroot: String) {
    let mut buffer = [0; 4096];
    stream.read(&mut buffer).unwrap();


    let get = b"GET /image.gif HTTP/1.1\r\n";
    let healtcheck = b"GET /healthcheck HTTP/1.1\r\n";
    let readycheck = b"GET /readycheck HTTP/1.1\r\n";

    if buffer.starts_with(healtcheck) {
        let response = format!("HTTP/1.1 200 OK\r\nServerd-by: my_rust_http_server\r\nContent-Length: 27\r\n\r\nPod is happy, so I´m happy");
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    if buffer.starts_with(readycheck) {
        let response = format!("HTTP/1.1 200 OK\r\nServerd-by: my_rust_http_server\r\nContent-Length: 12\r\n\r\nPod is ready");
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    if buffer.starts_with(get) {
        let status_line = "HTTP/1.1 200 OK";

        let mut r = rand::thread_rng();
        /*
        let files = fs::read_dir(docroot).expect("Cannot access directory").filter_map(|res| res.ok()).map(|dir_entry| dir_entry.path()).filter_map(|path| {
                if path.extension().map_or(false, |ext| ext == "gif") {
                    Some(path)
                } else {
                    None
                }
            }).collect::<Vec<_>>();
        */
        let files = fs::read_dir(docroot).expect("Cannot access directory").filter_map(|res| res.ok()).filter_map(|dir_entry| {
            let path = dir_entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "gif") {
                Some(path)
            } else {
                None
            }
            }).collect::<Vec<_>>();

        let file_index = r.gen_range(0..files.len());
        let file = files[file_index].display().to_string();

        let contents = fs::read(&file).unwrap();
        let response = format!("{}\r\nServerd-by: my_rust_http_server\r\nContent-Length: {}\r\nContent-Type: image/gif\r\n\r\n", status_line, contents.len());
        stream.write(response.as_bytes()).unwrap();
        stream.write(&contents).unwrap();
        stream.flush().unwrap();
        println!("Send {} to {}", file, stream.local_addr().unwrap());
    }
    else {
        let response = format!("HTTP/1.1 403 FORBIDDEN\r\nServerd-by: my_rust_http_server\r\nContent-Length: 13\r\n\r\n403 Forbidden");
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

fn return_listener_config (ip: String, port: String) -> String {
    let listener_config = [ip, port].join(":");
    listener_config
}

fn main() {
    let argumets = std::env::args().skip(1);
    let server_config: Config = Config::new_config(argumets);
    let listener_config = return_listener_config(server_config.ip, server_config.port.to_string());
    let listener = TcpListener::bind(listener_config).unwrap();
    let pool = ThreadPool::new(server_config.worker_count,);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let document_root = server_config.docroot.clone();
        pool.execute(move ||{
            handle_connection(stream, document_root);
        });
    }
    println!("Server is shutting down");
}
