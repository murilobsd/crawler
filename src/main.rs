pub struct Request<'a> {
    url: &'a str,
}

impl<'a> Request<'a> {
    pub fn new(url: &'a str) -> Request<'a> {
        Self { url }
    }
}

pub struct Site<'a> {
    user_agent: &'a str,
}

impl<'a> Site<'a> {
    pub fn new(user_agent: &'a str) -> Site<'a> {
        Self { user_agent }
    }
}

fn main() {
    let url = "http://httpbin.org/ip";
    let _ = Request::new(url);
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

    #[test]
    fn new_site_url_ok() {
        let user_agent = "crawler-rs/0.0.1";
        let site = Site::new(user_agent);
        assert_eq!(user_agent, site.user_agent);
    }
}
