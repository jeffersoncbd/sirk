## Resumo
Valida que `TREE_TOOL` seja a string `"allow"`, convertendo isso em um booleano.

## Funcionamento
A função atua como deserializador customizado de `bool`: lê o valor como `String` e, se for exatamente `"allow"`, retorna `Ok(true)`; qualquer outro valor gera um erro de desserialização com mensagem descritiva (`serde::de::Error::custom`), fazendo o parsing falhar em tempo de carregamento da configuração.

## Importações
- `serde::Deserialize`: Permite ler o valor bruto da configuração como `String`.
