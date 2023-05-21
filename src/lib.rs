pub fn handle(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(_length) => {
            println!("Connection established!");
        },
        Err(_err) => {
            println!("Something went wrong!");
        }
    }
}
