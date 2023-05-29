# trace_project
Projeto de criação de um gerador de carga baseado em traços de execução

## Dados do projeto

Projeto de desenvolvimento de uma aplicação geradora de carga baseada em traços de execução.

- Feito utilizando a linguagem rust

## Etapas para realização de um protótipo com fila

1. - [x] Obtenção de arquivo log de alguma database
2. - [x] Parsing deste arquivo para uma estrutura de dados
3. - [x] Realização das requisições baseado nesta estrutura de dados

## Refinando o protótipo

1. - [x] Implementar protótipo de heatmap em rust
2. - [] Refinar protótipo
3. - [] Integrar heatmap com a aplicação, substituindo a fila

## Sintaxe dos comandos utilizados pelo YCSB

### HGETALL
```
HGETALL key
```
- Example output:
```
redis> HSET myhash field1 "Hello"
(integer) 1
redis> HSET myhash field2 "World"
(integer) 1
redis> HGETALL myhash
1) "field1"
2) "Hello"
3) "field2"
4) "World"
```

### HSET
```
HSET key field value [field value ...]
```
- Example output:
```
redis> HSET myhash field1 "Hello"
(integer) 1
redis> HGET myhash field1
"Hello"
redis> HSET myhash field2 "Hi" field3 "World"
(integer) 2
redis> HGET myhash field2
"Hi"
redis> HGET myhash field3
"World"
redis> HGETALL myhash
1) "field1"
2) "Hello"
3) "field2"
4) "Hi"
5) "field3"
6) "World"
```

## Sintaxe do comando MONITOR

```
timestamp [client info] command
```
- Example:
```
1683138513.644266 [0 127.0.0.1:59998] "COMMAND" 
```

# Organizando implementação

## Sugestão Odorico

Um Heatmap para cada comando, com as colunas repesentando cada dado e as linhas representando o tempo, com cada casa sendo uma janela de observação
