use std::{ fs, net::{ TcpListener, TcpStream }, env, io::{ Write, Read } };

fn main() {
    const PORT: i16 = 4000;
    let server = TcpListener::bind(format!("127.0.0.1:{}", PORT)).unwrap();

    for stream in server.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    stream.read(&mut vec![0; 1024]).unwrap();

    let status_line = "HTTP/1.1 200 OK";
    let headers = "Content-Type: text/html";

    let content_buffer = fs
        ::read(format!("{}\\{}", env::current_dir().unwrap().to_str().unwrap(), "test.txt"))
        .unwrap();

    let content = String::from_utf8(content_buffer).unwrap();

    let response = format!("{status_line}\r\n{headers}\r\n\r\n{content}");

    stream.write_all(response.as_bytes()).unwrap();
}
