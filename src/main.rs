use crud_app::handle;
use std::net::TcpListener;

fn main() {
    let address = "0.0.0.0:3000";
    let listener = TcpListener::bind(address).unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle(stream);
            }
            Err(err) => {
                println!("Failed to connect to {address}. {err}");
            }
        }
    }
}
