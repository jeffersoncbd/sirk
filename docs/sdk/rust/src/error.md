## Resumo
Define os erros que o SDK pode retornar ao interagir com o sistema.

## Funcionamento
Representa falhas de entrada e saída e de JSON, convertidas automaticamente; erros de protocolo com descrição e erros RPC remotos com código e mensagem.

## Importações
- `thiserror`: Gera a implementação de `std::error::Error`.
- `std::io`: Fornece o tipo de erro de entrada e saída.
- `serde_json`: Fornece o tipo de erro de serialização JSON.
