pub struct Request<'a> {
    url: &'a str
}

impl<'a> Request<'a> {
    pub fn new(url: &'a str) -> Request<'a> {
        Self{url}
    }
}

fn main() {
    let url = "http://httpbin.org/ip";
    let _ = Request::new(url);
}
