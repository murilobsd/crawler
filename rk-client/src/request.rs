// Copyright (c) 2020 Murilo Ijanc' <murilo.ijanc@kovi.com.br>
//
// Permission to use, copy, modify, and distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

use std::fmt;

use super::client;
use super::RakunMethod;
use url;

pub struct RakunRequest {
    pub in_method: RakunMethod,
    pub uri: url::Url,
}

impl RakunRequest {
    pub fn new<S: AsRef<str>>(
        in_method: RakunMethod,
        uri: S,
    ) -> Result<RakunRequest, url::ParseError> {
        let u = url::Url::parse(uri.as_ref()).unwrap();
        Ok(Self { in_method, uri: u })
    }

    #[inline]
    pub fn url(&self) -> url::Url {
        self.uri.clone()
    }

    #[inline]
    pub fn method(&self) -> RakunMethod {
        self.in_method.clone()
    }

    pub(crate) fn into_client(
        self,
        client: &client::RakunClient,
    ) -> Result<reqwest::blocking::RequestBuilder, ()> {
        let req_builder = client.http.request(self.in_method, self.uri);

        Ok(req_builder)
    }
}

impl fmt::Display for RakunRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<Request: {} {}>",
            self.in_method.as_ref(),
            self.uri.as_ref()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_request_test() {
        let url = "http://httpbin.org/";
        let req = RakunRequest::new(RakunMethod::GET, url).unwrap();
        assert_eq!("<Request: GET http://httpbin.org/>", format!("{}", req));
    }

    #[test]
    fn request_get() {
        let url = "http://httpbin.org/";
        let req = RakunRequest::new(RakunMethod::GET, url).unwrap();
        assert_eq!(url, req.url().as_str());
        assert_eq!("GET", req.method().as_str());
    }
}
