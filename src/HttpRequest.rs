use std::collections::HashMap;
use std::net::TcpStream;
use std::io::{BufRead, BufReader, Read, Write};
use std::borrow::Borrow;
use std::ops::Deref;
use std::ptr::null;


pub struct HttpRequest {
    pub requestType:    HttpRequestType,
    headers:            HashMap<String, String>,
    pub path:           String,
    myStream:           TcpStream
}

pub enum HttpRequestType {
    GET
}

fn  getRequestType(line: String ) ->HttpRequestType{
    let result = match line.split_whitespace().next().unwrap_or(""){
        "GET" => {HttpRequestType::GET},
        _ =>     HttpRequestType::GET

    };
    HttpRequestType::GET

}
fn  getPath(line: String ) ->String{
    println!("line: {}",line);
    return String::from(line.split_whitespace().nth(1).unwrap());

}

impl HttpRequest{
    pub fn new(stream: TcpStream) ->HttpRequest{
        let mut headers: HashMap<String, String> = HashMap::new();
        let mut reader = BufReader::new(&stream);

        let mut buf = Vec::new();

        let mut size = reader.read_until(b'\n', &mut buf).unwrap();

        let reqType:HttpRequestType = getRequestType(String::from(String::from_utf8_lossy(&buf)));
        println!("{}", String::from_utf8_lossy(&buf));
        let path = getPath(String::from(String::from_utf8_lossy(&buf)));

        while size  > 0 {
            buf.clear();
            size = reader.read_until(b'\n', &mut buf).unwrap();
            if size == 2 && &buf == b"\r\n" {
                break;
            }
            let header = String::from_utf8_lossy(&mut buf);
            let split: Vec<&str> = header.split(':').collect();

            headers.insert(String::from(split[0]),String::from(split[1]));
        }

        return HttpRequest {requestType: reqType,headers: headers,path: path, myStream: stream};

    }

    pub fn sendOk(&mut self, contents: String){
        let mut response = String::from ("HTTP/1.1 200\r\n");
        response.push_str("Content-Type: text/html; charset=UTF-8\r\n");
        response.push_str("Content-Length: ");
        response.push_str((contents.as_bytes().len() as u32).to_string().as_str());
        response.push_str("\r\n\r\n");
        response.push_str(contents.as_str());



        self.myStream.write(response.as_bytes()).unwrap();
        self.myStream.flush();
    }

    pub fn send404(&mut self) {


        let contents = String::from("Page does not exist.");

        let mut response = String::from ("HTTP/1.1 404\r\n");
        response.push_str("Content-Type: text/html; charset=UTF-8\r\n");
        response.push_str("Content-Length: ");
        response.push_str((contents.as_bytes().len() as u32).to_string().as_str());
        response.push_str("\r\n\r\n");
        response.push_str(contents.as_str());

        self.myStream.write(response.as_bytes()).unwrap();
        self.myStream.flush();
    }


}
