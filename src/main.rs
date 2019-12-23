use std::io::{BufRead, BufReader, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream, IpAddr, Shutdown};
use std::collections::HashMap;

mod HttpRequest;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();

    loop {
        match listener.accept() {
            Ok((stream, addr)) => handle_request(stream, addr),
            Err(e) => println!("An error occured: {:?}", e)
        }
    }
}
    fn handle_request(mut stream: TcpStream, addr: SocketAddr) {
        //let x = stream.write(b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n").unwrap();
//        stream.flush().unwrap();
//        stream.shutdown(Shutdown::Both);
        let mut request = HttpRequest::HttpRequest::new(stream);
        //sendOk(stream, "<html><body>Hello world</body></html>\r\n".parse().unwrap());
        if (request.path.contains("test")) {
            request.send404();
        }
        else {
            request.sendOk("<html><body>Hello world</body></html>\r\n".parse().unwrap())
        }

    }





