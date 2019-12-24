use std::collections::HashMap;
use std::net::TcpStream;
use std::io::{BufRead, BufReader, Read, Write};
use std::borrow::Borrow;
use std::ops::Deref;
use std::ptr::null;


pub struct HttpRequest {
    pub request_type:    HttpRequestType,
    headers:            HashMap<String, String>,
    pub path:           String,
    my_stream:           TcpStream
}

pub enum HttpRequestType {
    GET
}


impl HttpRequest{
    pub fn new(stream: TcpStream) ->HttpRequest{
        let mut headers: HashMap<String, String> = HashMap::new();
        let mut reader = BufReader::new(&stream);

        let mut buf = Vec::new();

        let mut size = reader.read_until(b'\n', &mut buf).unwrap();

        let request_type:HttpRequestType = HttpRequest::get_request_type(String::from(String::from_utf8_lossy(&buf)));
        let path = HttpRequest::get_path(String::from(String::from_utf8_lossy(&buf)));

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

        return HttpRequest { request_type: request_type,headers: headers,path: path, my_stream: stream};

    }

    fn get_path(line: String ) ->String{
        return String::from(line.split_whitespace().nth(1).unwrap_or(""));

    }


    fn get_request_type(line: String ) ->HttpRequestType{
        return match line.split_whitespace().next().unwrap_or(""){
            "GET" => {HttpRequestType::GET},
            _ =>     HttpRequestType::GET

        };
    }


    pub fn send_ok(&mut self, contents: String){
        let mut response = String::from ("HTTP/1.1 200\r\n");
        response.push_str("Content-Type: text/html; charset=UTF-8\r\n");
        response.push_str("Content-Length: ");
        response.push_str((contents.as_bytes().len() as u32).to_string().as_str());
        response.push_str("\r\n\r\n");
        response.push_str(contents.as_str());



        self.my_stream.write(response.as_bytes()).unwrap();
        match self.my_stream.flush(){
            Ok(_x)=>{},
            Err(e) =>println!("Error on flush: {:?}",e)

        }
    }

    pub fn send_404(&mut self) {


        let contents = String::from("Page does not exist.");

        let mut response = String::from ("HTTP/1.1 404\r\n");
        response.push_str("Content-Type: text/html; charset=UTF-8\r\n");
        response.push_str("Content-Length: ");
        response.push_str((contents.as_bytes().len() as u32).to_string().as_str());
        response.push_str("\r\n\r\n");
        response.push_str(contents.as_str());

        self.my_stream.write(response.as_bytes()).unwrap();
        match self.my_stream.flush(){
            Ok(_x)=>{},
            Err(e) =>println!("Error on flush: {:?}",e)

        }
    }


}
