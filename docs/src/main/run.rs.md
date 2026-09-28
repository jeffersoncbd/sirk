## Resumo
Executa o comando solicitado a partir dos argumentos recebidos.

## Funcionamento
Sem argumentos ou com `help`, exibe a ajuda; reconhece aliases para criação de agente e aceita `http` com ou sem endereço, além de `rpc`. Para qualquer outra combinação, retorna um erro com a mensagem de uso.

## Importações
- `create_agent`: Cria um agente.
- `http`: Executa o comando HTTP.
- `print_usage`: Exibe a ajuda.
- `rpc`: Executa o comando RPC.
- `usage`: Fornece a mensagem de uso.
