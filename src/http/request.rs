pub struct Request {
    method: super::method::Method,
    query_string: Option<String>,
    path: String,
}
