use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use tcp::ThreadPool;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming(){
        let stream  = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });        
    }
}

fn handle_connection(mut stream: TcpStream){
    let buf_reader = BufReader::new(&mut stream);
    let _http_req = buf_reader
        .lines()
        .next().unwrap().unwrap();

    if _http_req == "GET / HTTP/1.1"{
        let contents = fs::read_to_string("index.html").unwrap();
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
        thread::sleep(Duration::from_secs(5));
        stream.write_all(response.as_bytes()).unwrap();

    }else if _http_req == "GET /login HTTP/1.1"{
        let contents = fs::read_to_string("login.html").unwrap();
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
        stream.write_all(response.as_bytes()).unwrap(); 

    }else{    
        let status_line = "HTTP/1.1 400 NOT FOUND";
        let contents = fs::read_to_string("error.html").unwrap();
        let length = contents.len();
        let http_res = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
        stream.write_all(http_res.as_bytes()).unwrap();
    }
}
