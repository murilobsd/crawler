# Rakun

[![CI Tests](https://img.shields.io/badge/BUILD-PASSING-green?style=for-the-badge)](https://github.com/murilobsd/rakun/actions?query=workflow%3A%22CI+Tests%22) [![license](https://img.shields.io/badge/LICENSE-ISC-blue?style=for-the-badge)](LICENSE)

## O que é o Rakun?

Rakun é um projeto para coleta de multas de trânsito de diversas fontes do
Brasil.  O objetivo é descomplicar o acesso a essas informações realizando de
forma rápida, transparente e segura as consultas a essas fontes.

## Problema

Atualmente para uma pessoa física consultar multas de trânsito associadas a seu
veículo é muito complicado, quando falamos de uma locadora multiplicamos esse
problema pelo número de carros ativos na mesma. Algumas dificuldades são:

- Instabilidade do sistema (latência, indisponibilidade, website fora de
    padrões internacionais...)
- Anti-Robo
- Links quebrados
- Autenticações
- Formulários gigantes
- Fontes não possuem um padrão de consulta (umas pedem número do renavam,
    outras pedem número da placa...)

## Solução

Criar um sistema distribuído para notificar as multas existentes do veículo em
tempo hábil (menor do que 15 dias) para que locadoras consigam assim efetivar
todo o processo envolvido na indicação da multa e cobrança do motorista.

As notificações podem ser de diversas formas:

- Webhook
- Email
- Sms
- Bot Slack
