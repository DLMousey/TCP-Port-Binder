use std::net::TcpListener;

fn main() {

    let tcp_listener = TcpListener::bind("127.0.0.1:32500");
    let _tcp_listener = match tcp_listener {
        Ok(tcpl) => bind_incoming(tcpl),
        Err(err) => {
            panic!("Unable to bind to port 32500! Error was: {:?}", err)
        }
    };
}

fn bind_incoming(listener: TcpListener) {
    println!("\r\n ---------- \r\n TCP Listener listening on port 32500...");
    for stream in listener.incoming() {
        let _stream = stream.unwrap();

        println!("Connection established from: {:?}", _stream.local_addr());
    }
}
