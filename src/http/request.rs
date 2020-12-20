pub struct Request<'a> {
    pub url: &'a str,
}

impl<'a> Request<'a> {
    pub fn new(url: &'a str) -> Request<'a> {
        Self { url }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_request_url_ok() {
        let url = "http://httpbin.org/ip";
        let req = Request::new(url);
        assert_eq!(url, req.url);
    }
}
