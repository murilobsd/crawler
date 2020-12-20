pub struct Site<'a> {
    pub user_agent: &'a str,
}

impl<'a> Site<'a> {
    pub fn new(user_agent: &'a str) -> Site<'a> {
        Self { user_agent }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_site_url_ok() {
        let user_agent = "crawler-rs/0.0.1";
        let site = Site::new(user_agent);
        assert_eq!(user_agent, site.user_agent);
    }
}

