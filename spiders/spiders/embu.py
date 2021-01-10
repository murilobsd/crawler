# Copyright (c) 2021 Murilo Ijanc' <mbsd@m0x.ru>
#
# Permission to use, copy, modify, and distribute this software for any
# purpose with or without fee is hereby granted, provided that the above
# copyright notice and this permission notice appear in all copies.
#
# THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
# WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
# MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
# ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
# WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
# ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
# OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
from typing import Union

import aiohttp
import asyncio

class ExceptionRenavam(Exception):
    pass

class Renavam:
    def __init__(self, renavam_number: Union[int, str]):
        if isinstance(renavam_number, int):
            renavam_sz = len(str(renavam_number))
        elif isinstance(renavam_number, str):
            renavam_sz = len(renavam_number)
        else:
            raise ExceptionRenavam("renavam type: %s" % type(renavam_number))

        if renavam_sz < 9:
            raise ExceptionRenavam("len renavam got: %d - 9" % renavam_sz)

        self.data = renavam_number

class Client:
    def __init__(self):
        conn = aiohttp.TCPConnector(limit_per_host=30, ttl_dns_cache=300)
        self.session = aiohttp.ClientSession(connector=conn)

    def timeout(self):
        pass

    def proxy(self):
        pass

    def captcha(self):
        pass

    async def close(self):
        await self.session.close()


class SpiderEmbu:
    base_url = "http://sistemas.cobrasin.com.br{}"
    url_page = base_url.format("/multas-municipe/home.action?municipio=8");
    url_form = base_url.format("/multas-municipe/pesquisaMultaMunicipe/pesquisar.action")
    municipio = "8"

    def __init__(self, client: Client, renavam: Renavam):
        self.client = client
        self.renavam = renavam

    async def init(self):
        await self.client.session.get(self.url_page)

    def form(self):
        return {
            "municipio": self.municipio,
            "renavam": self.renavam.data
        }

    async def multa(self):
        await self.init()
        resp = await self.client.session.post(self.url_form, data=self.form())
        print(await resp.text())
        await self.client.close()

if __name__ == "__main__":
    import os
    rn = os.getenv("RENAVAM", "0000000000")

    renavam = Renavam(rn)
    client = Client()
    spider = SpiderEmbu(client, renavam)

    loop = asyncio.get_event_loop()
    loop.run_until_complete(spider.multa())
    loop.close()
