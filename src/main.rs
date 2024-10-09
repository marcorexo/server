fn main() {

    let string: String = String::from("127.0.0.1:8080");
    let string_slice: &str = &string[10..];

    println!("address is: {} ", &string);
    println!("sliced address: {}", &string_slice);


    //let server = Server::new("127.0.0.1:8080");
    //server.run();
}

