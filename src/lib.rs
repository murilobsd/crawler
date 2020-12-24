use hyper;

pub mod http {
    pub struct Request<'a> {
        url: &'a str,
        method: hyper::method::Method,
    }
}

pub mod spider {
    /// This module is responsible for parsing the response.
    pub struct Spider<'a> {
        pub name: &'a str,
        pub start_urls: Vec<&'a str>,
    }

    impl<'a> Spider<'a> {
        pub fn new(name: &'a str) -> Spider<'a> {
            Spider {
                name,
                start_urls: Vec::new(),
            }
        }

        pub fn push_url(&mut self, s: &'a str) {
            self.start_urls.push(s)
        }

        pub fn start_requests(&self) {
            for url in &self.start_urls {
                println!("Request {}", url);
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::Spider;

        #[test]
        fn spider_new_test() {
            let sp = Spider::new("rakun");
            assert_eq!("rakun", sp.name);
        }

        #[test]
        fn spider_push_url_test() {
            let url = "http://httpbin.org/get";
            let mut sp = Spider::new("rakun");

            sp.push_url(url);

            assert_eq!(1, sp.start_urls.len());
        }
    }
}

// pub mod crawler {
//     use std::collections::HashSet;

//     use crate::spider::Spider;

//     /// Crawler struct
//     pub struct Crawler {
//         crawling: bool,
//     }

//     impl Crawler {
//         pub fn new() -> Self {
//             Self {crawling: false}
//         }

//         pub fn crawl() {}

//         fn create_engine(&self) {
//             println!("Create Execution Engine");
//         }

//         fn create_spider(&self) {
//             println!("Create Spider");
//         }
//     }

//     /// Run crawlers
//     pub struct CrawlerRunner {
//         pub crawlers: HashSet<Crawler>,
//         pub active: HashSet<Crawler>,
//     }

//     // TODO: carrager o spider_loader aqui dentro
//     impl CrawlerRunner {
//         pub fn new() -> Self {
//             Self {
//                 crawlers: HashSet::new(),
//                 active: HashSet::new(),
//             }
//         }

//         pub fn create_crawler(s: Spider) {
//             println!("init spider {}", s.name);
//         }
//     }
// }
