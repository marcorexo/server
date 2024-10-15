fn main() {
    /*
    let string: String = String::from("127.0.0.1:8080");
    let string_slice: &str = &string[10..];

    println!("address is: {} ", &string);
    println!("sliced address: {}", &string_slice);
    */
    //let string: String = String::from("127.0.0.1:8080".to_string());
    //let string: String = String::from("127.0.0.1:8080");
    //let string_slice: &str = &string[10..];

    let server: Server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        //Self (upper case 's') and Server is the same
        Self { addr }
    }

    fn run(self) {
        println!("Listening on {}", &self.addr);
    }
}

struct Request {
    path: String,
    query_string: String,
    method: Method,
}

enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    OPTIONS,
    HEAD,
    CONNECT,
    TRACE,
    PATCH,
}

/*
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/
