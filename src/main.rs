use std::io::{BufRead, BufReader, Read, Write, ErrorKind, Error};
use std::net::{SocketAddr, TcpListener, TcpStream, IpAddr, Shutdown};
use std::collections::HashMap;

use std::path::Path;
use std::fs::File;

use std::env;
use std::borrow::Borrow;

mod HttpRequest;
mod Configuration;

#[macro_use]
extern crate clap;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref HTML_DIR: String = {String::from(env::current_dir().unwrap().to_str().unwrap())};
    static ref CONFIG: Configuration::Config = Configuration::Config::new();
}

fn main() {

    let path = env::current_dir();

    println!("The current directory is {}", CONFIG.html_path);


    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();

    loop {
        match listener.accept() {
            Ok((stream, addr)) => handle_request(stream, addr),
            Err(e) => println!("An error occured: {:?}", e)
        }
    }
}
    fn handle_request(mut stream: TcpStream, addr: SocketAddr) {
        let mut request = HttpRequest::HttpRequest::new(stream);
        match get_file_by_path(&request.path) {
            Ok(content) => request.sendOk(content),
            Err(e) => request.send404()
        }

    }

    fn get_file_by_path(path: &String) -> Result<String, Error>{
        let filePath = Path::new(&CONFIG.html_path).join(&path[1..]);
        println!("{}",CONFIG.html_path);
        println!("{}",filePath.to_str().unwrap());

        if (filePath.exists()){
            println!("exists");
            if (filePath.is_file()){
                let mut file = File::open(filePath.to_str().unwrap()).unwrap();
                let mut contents = String::new();
                file.read_to_string(&mut contents);
                return Ok(contents)
            }
            if (filePath.is_dir()){
                println!("dir");
                let indexPath = filePath.join("index.html");
                if (indexPath.is_file()){
                    let mut file = File::open(indexPath.to_str().unwrap()).unwrap();
                    let mut contents = String::new();
                    file.read_to_string(&mut contents);
                    return Ok(contents)
                }
            }
        }

        return Err(Error::new(ErrorKind::Other,"404"));



    }





