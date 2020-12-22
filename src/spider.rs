pub struct Spider<'a> {
    pub name: &'a str,
    pub start_urls: dyn AsRef<str>
}
