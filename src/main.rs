use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3000").unwrap();
    println!("running in 3000......");

    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        println!("Connection established!");
    }
}

