use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

use server::ThreadPool;


//start server
fn main() {
    let listner = 
        TcpListener::bind("127.0.0.1:7878").unwrap();  //Port

    let pool = ThreadPool::new(4);  //Set number of max threads

    for stream in listner.incoming(){
        let stream = stream.unwrap();

        //1 thread per request
        pool.execute(|| {
            handle_connection(stream);
        });
        
    }
}

//handle connections
fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";  //Server Mux(ig)

    if buffer.starts_with(get){
        let contents = fs::read_to_string("index.html").unwrap();

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        stream.write(response.as_bytes()).unwrap();  //send resonse
        stream.flush().unwrap();
    }else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("notfound.html").unwrap();

        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        );
        stream.write(response.as_bytes()).unwrap();  //send resonse
        stream.flush().unwrap();   
    }


}