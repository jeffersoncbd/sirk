## Resumo
Despacha os argumentos para o comando correspondente ou exibe a ajuda.

## Funcionamento
Sem argumentos ou com `help`, `--help` ou `-h`, exibe a ajuda e retorna sucesso. Reconhece `--newAgent` e `--new-agent` para criar um agente, e `http` com endereço opcional. Para qualquer outra combinação, retorna um erro com a mensagem de uso.

## Importações
- `create_agent`: Cria um agente.
- `http`: Executa o comando HTTP.
- `print_usage`: Exibe a ajuda.
- `usage`: Fornece a mensagem de uso.
