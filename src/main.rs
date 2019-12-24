#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;

use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read, Write};
use std::net::{IpAddr, Shutdown, SocketAddr, TcpListener, TcpStream};
use std::path::Path;

mod HttpRequest;
mod Configuration;

lazy_static! {
    static ref CONFIG: Configuration::Config = Configuration::Config::new();
}

fn main() {

    let listener = TcpListener::bind(CONFIG.url).unwrap();

    loop {
        match listener.accept() {
            Ok((stream, addr)) => handle_request(stream, addr),
            Err(e) => println!("An error occured: {:?}", e)
        }
    }
}
    fn handle_request(stream: TcpStream, addr: SocketAddr) {
        let mut request = HttpRequest::HttpRequest::new(stream);
        match get_file_by_path(&request.path) {
            Ok(content) => request.send_ok(content),
            Err(_e) => request.send_404()
        }

    }

    fn get_file_by_path(path: &String) -> Result<String, Error>{
        let file_path = Path::new(&CONFIG.html_path).join(&path[1..]);

        if file_path.exists() {
            if file_path.is_file(){
                let mut file = File::open(file_path).unwrap();
                let mut contents = String::new();
                return match file.read_to_string(&mut contents){
                    Ok(_x) => Ok(contents),
                    Err(_e) =>Err(Error::new(ErrorKind::Other,"404"))
                }
            }
            if file_path.is_dir() {
                let index_path = file_path.join("index.html");
                if index_path.is_file(){
                    let mut file = File::open(index_path).unwrap();
                    let mut contents = String::new();
                    return match file.read_to_string(&mut contents){
                        Ok(_x) => Ok(contents),
                        Err(_e) =>Err(Error::new(ErrorKind::Other,"404"))
                    }
                }
            }
        }

        return Err(Error::new(ErrorKind::Other,"404"));



    }





