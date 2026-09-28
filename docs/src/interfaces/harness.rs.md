## Resumo
Define os tipos e a interface para executar agentes de programação por diferentes adaptadores.

## Funcionamento
`HarnessAdapter` fornece um identificador e converte uma solicitação em uma invocação, retornando erros para opções ou configurações inválidas; por padrão, `response` repassa a saída padrão sem alterações. `RunRequest` reúne os dados da execução, e `HarnessError` representa falhas na integração.

## Importações
- `crate::interfaces::Invocation`: Representa a chamada ao agente.
- `std::path::PathBuf`: Armazena o diretório de trabalho.
