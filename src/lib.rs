pub mod spider {
    /// This module is responsible for parsing the response.
    pub struct Spider<'a> {
        pub name: &'a str,
        pub start_urls: Vec<&'a str>
    }

    impl<'a> Spider<'a> {
        pub fn new(name: &'a str, start_urls: Vec<&'a str>) -> Self {
            Self { name , start_urls }
        }

        pub fn push_url(&mut self, s: &'a str) {
            self.start_urls.push(s)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::Spider;

        #[test]
        fn spider_new_test() {
            let urls = vec!["http://httpbin.org/ip"];
            let sp = Spider::new("rakun", urls);

            assert_eq!("rakun", sp.name);
            assert_eq!(1, sp.start_urls.len());
        }

        #[test]
        fn spider_push_url_test() {
            let urls = vec!["http://httpbin.org/ip"];
            let url = "http://httpbin.org/get";
            let mut sp = Spider::new("rakun", urls);

            sp.push_url(url);

            assert_eq!(2, sp.start_urls.len());
        }
    }
}
