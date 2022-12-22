use std::{fs, io::{BufRead, BufReader}, net::{TcpListener, TcpStream}};
use std::io::Write;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6969").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream)
    }
}
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    /*
    format of a http response
        HTTP-Version Status-Code Reason-Phrase CRLF
        headers CRLF
        message-body
    */
    //this lie gets us the first line of the http response (the request line) rather than the whole
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        // this handles request from the URL with '/' URI
        ("HTTP/1.1 200 OK", "hello.html")
    }else {
        // this handles request from any other URI
        ("HTTP/1.1 404 PageNotFound", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();


    /*
    //this fetches the http_request details
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result|result.unwrap())
        .take_while(|line|!line.is_empty())
        .collect();
    */




}
