fn main() {
    let get = Method::GET;
    let post = Method::POST;
    let put = Method::PUT;
    let delete = Method::DELETE;
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self { address }
    }

    fn run(self) {
        println!("Listening on {}", self.address);
    }
}

struct Request {
    method: Method,
    query_string: Option<String>,
    path: String,
}

enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    HEAD,
    TRACE,
    CONNECT,
    OPTIONS,
}
