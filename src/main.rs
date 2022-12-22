use std::{fs, io::{BufRead, BufReader}, net::{TcpListener, TcpStream}, thread};
use std::io::Write;
use std::time::Duration;
use hello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6969").unwrap();
    let pool = ThreadPool::new(4); //create a thread pool with finite number of threads, 4

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(||{
            handle_connection(stream)
        });

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
    //this gets us the first line of the http response (the request line) rather than the whole
    let request_line = buf_reader.lines().next().unwrap().unwrap();


    // this handles request from the URL with '/' and '/sleep' URI to return hello.html and 404.html for other URIs
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 PageNotFound", "404.html")
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
