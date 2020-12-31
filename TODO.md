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

## roadmap zoom sem captcha

1. checa se existe captcha para essa fonte
2. obtem um proxy de uma lista
3. checa a disponibilidade do proxy (velocidade)
4. esta ok continue se nao testa outro proxy
5. set o user-agent
6. set o timeout
7. acessa o website (set cookies)
8. checa
8.1 - status code
8.2 - formulario campos estao ok
9. tudo ok com o formulario continue se nao envia um altera de alteracao do
website.
10. submete o formulario
11. checa o status code e resposta se tiver ok continue se nao relate o problema
12. obtem os dados da multa se estiver ok, salva e notifica o cliente, relata o
problema no db

10 crawlers (detrans/municipais) atualmente 192 fontes
10 inputs (renavans + placa) -> async


```
let inputs = vec![(Renavam, Placa), (Renavam1, Placa1)];
send_to_workers(inputs, topic);
```

```
receive_job(job);
```
