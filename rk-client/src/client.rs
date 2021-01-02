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

use super::request;

static USER_AGENT: &str = concat!("rakun", "/", env!("CARGO_PKG_VERSION"),);

pub struct RakunClient {
    pub http: reqwest::blocking::Client,
}

impl RakunClient {
    pub fn new() -> Self {
        let client = reqwest::blocking::Client::builder()
            .danger_accept_invalid_certs(true)
            .no_proxy()
            .user_agent(USER_AGENT)
            .build()
            .unwrap();
        Self { http: client }
    }

    pub fn execute(
        &self,
        req: request::RakunRequest,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let resp = req.into_client(self).unwrap();

        Ok(resp.send()?)
    }
}

impl Default for RakunClient {
    fn default() -> Self {
        Self::new()
    }
}

pub fn rakun_client() -> Result<reqwest::Client, ()> {
    let client = reqwest::Client::new()
        .cookie_store(true)
        .build()?
    client
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
