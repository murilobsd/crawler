# Models

## Job

|Name|Type|Description|
|----|----|-----------|
|client id|alfanum|uuid4 identification client|
|nrmv|uint|national registry of motor vehicles (RENAVAM)|
|plate number|alfanum|license plate number|

## Ticket

|Name|Type|Description|
|----|----|-----------|
|[typing ticket][1]|enum|coding tickets|
|Address|object||
|Date|date||
|time|time||
|plate number|alfanum|
|brand|alfanum||
|model|alfa||
|information driver|object||
|authority id|alfanum||
|signature driver|alfanum||

[1]: <https://www.detran.pa.gov.br/cetran/infracao/pdf/portaria_59_detran.pdf> "Tabela codificação das multas"
