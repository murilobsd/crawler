# TODO

## Crawler (client http)

- [ ] Cookies
- [x] Http and Https
- [ ] Delay per request
- [ ] Proxy + RoudRobin
- [x] Random User-Agent
- [ ] Retry
- [ ] Support break captchas
- [ ] Workers
- [ ] Checking if content of sources change
- [ ] Downtime of sources

## roadmap

1. adicionado novo veiculo
2. publica no topico
3. worker obtem checa em todas as fontes
4. salva no db

10 crawlers (detrans/municipais) atualmente 192 fontes
10 inputs (renavans + placa) -> async

```
let inputs = vec![(Renavam, Placa), (Renavam1, Placa1)];
send_to_workers(inputs, topic);
```

```
receive_job(job);
```
