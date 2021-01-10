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
import aiohttp
import asyncio

class ExceptionRenavam(Exception):
    pass

class Renavam:
    def __init__(self, renavam_number):
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

    async def close(self):
        await self.session.close()

class SpiderEmbu:
    base_url = "http://sistemas.cobrasin.com.br{}"
    municipio = "8"

    def __init__(self, client):
        self.client = client

    async def multa(self, renavam):
        form = {
            "municipio": self.municipio,
            "renavam": renavam.data
        }
        url = self.base_url.format("/multas-municipe/home.action?municipio=8")
        url_post = self.base_url.format("/multas-municipe/pesquisaMultaMunicipe/pesquisar.action")
        await self.client.session.get(url)
        resp = await self.client.session.post(url_post, data=form)
        print(resp.status)
        print(await resp.text())
        await self.client.close()

if __name__ == "__main__":
    import os
    rn = os.getenv("RENAVAM", "0000000000")
    renavam = Renavam(rn)
    client = Client()
    spider = SpiderEmbu(client)
    loop = asyncio.get_event_loop()
    loop.run_until_complete(spider.multa(renavam))
    loop.close()
