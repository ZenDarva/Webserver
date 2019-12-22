use std::io::{BufRead, BufReader, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream, IpAddr, Shutdown};
use std::collections::HashMap;

mod HttpRequest;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();

    loop {
        match listener.accept() {
            Ok((stream, addr)) => testFunc(stream, addr),
            Err(e) => println!("An error occured: {:?}", e)
        }
    }
}
    fn testFunc(mut stream: TcpStream, addr: SocketAddr) {
        //let x = stream.write(b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n").unwrap();
//        stream.flush().unwrap();
//        stream.shutdown(Shutdown::Both);
        sendOk(stream, "<html><body>Hello world</body></html>\r\n".parse().unwrap());
    }

    fn sendOk(mut stream: TcpStream, contents: String){
        let mut response = String::from ("HTTP/1.1 200\r\n");
        response.push_str("Content-Type: text/html; charset=UTF-8\r\n");
        response.push_str("Content-Length: ");
        response.push_str((contents.as_bytes().len() as u32).to_string().as_str());
        response.push_str("\r\n\r\n");
        response.push_str(contents.as_str());

        let request:HttpRequest::HttpRequest = HttpRequest::HttpRequest::new(&stream);
        //let mut headers = readHeaders( &stream);

        println!("{}",request.path);
        stream.write(response.as_bytes()).unwrap();
        stream.flush();
    }



