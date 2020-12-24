pub mod spider {
    /// This module is responsible for parsing the response.
    pub struct Spider<'a> {
        pub name: &'a str,
    }

    impl<'a> Spider<'a> {
        pub fn new(name: &'a str) -> Spider<'a> {
            Spider { name }
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
    }
}
