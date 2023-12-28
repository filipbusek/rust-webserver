use std::net::TcpListener;
use std::path::Path;

pub struct Config {
    pub ip: String,
    pub port: u16,
    pub docroot: String,
    pub worker_count: usize,
}

impl Config {
    pub fn new_config(arguments: std::iter::Skip<std::env::Args>) -> Config {
        let mut set_ip: bool = false;
        let mut ip: String = String::from("0.0.0.0");
        let mut set_port_number: bool = false;
        let mut port_number: u16 = 8080;
        let mut set_document_root: bool = false;
        let mut docroot: String = String::from("/var/www/html");
        let mut set_worker_count: bool = false;
        let mut worker_count: usize = 4;
        

        for argument in arguments {
            if set_ip == true {
                let test_ip = String::from(&argument);
                let test_ip_port_string = [test_ip, "8080".to_string()].join(":");
                ip = match TcpListener::bind(test_ip_port_string) {
                    Ok(_) => argument,
                    Err(e) => {
                        println!("Failed to bind ip {} with error:\n\t{}", argument, e);
                        std::process::exit(0x0002);
                    }
                };
                set_ip = false;
                continue;
            }

            if set_port_number == true {
                port_number = match argument.trim().parse() {
                    Ok(n) => n,
                    Err(..) => {
                        println!("{} is not valid port number", argument);
                        std::process::exit(0x0002);
                    }
                };
                set_port_number = false;
                continue;
            }

            if set_worker_count == true {
                worker_count = match argument.trim().parse() {
                    Ok(n) => {
                        match n {
                            1..=256 => n,
                            _ => {
                            println!("Worker count {} doesn´t make sense. Must be between 1-256\nExiting", argument);
                            std::process::exit(0x0002);
                            },
                        }
                    }
                    Err(..) => {
                        println!("{} is not valid number of workers. Must be number between 1-256", argument);
                        std::process::exit(0x0002);
                    }
                };
                set_worker_count = false;
                continue;
            }

            if set_document_root == true {
                docroot = match Path::new(&argument).is_dir() {
                    true => argument,
                    false => {
                        println!("{}, is not existing directory\nUse --root or -r parameter to specify valid directory", argument);
                        std::process::exit(0x0002);
                    }
                };
                set_document_root = false;
                continue;
            }

            match argument.as_str() {
                "--port"|"-p" => {
                    set_port_number = true;
                }
                "--ip" => {
                    set_ip = true;
                }
                "--worker"|"-w" => {
                    set_worker_count = true;
                }
                "--root"|"-r" => {
                    set_document_root = true;
                }
                "--help"|"-h" => {
                    println!("Current version: 0.1.3
Rust webserver which will return random gif file from specified folder

How to use:
    --root, -r: Specifies webroot in which will webserver be looking for gifs
    --port, -p: Specifies port for incoming connections
    --ip: Specifies on which ip should webserver listen for connections
    --worker, -w: Specifies number of available workers. Default is 4. Max value is 256
    --help, -h: Shows this message
");
                    std::process::exit(0x0000);
                }
                _ => {
                    println!("Invalid argument: {}\nUse -h or --help to get available arguments\n", argument);
                    std::process::exit(0x0002);
                }
            };
        }
        return Config { port: (port_number), docroot: (docroot), ip: (ip), worker_count: (worker_count) };
    }
}
