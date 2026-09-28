## Resumo
O adaptador configura a execução de solicitações pelo comando `codex exec`.

## Funcionamento
`invocation` monta os argumentos com acesso somente de leitura e aprovação desativada; inclui saída JSON e modelo quando solicitados, e acrescenta o prompt após `--`. Retorna a invocação com executável e diretório de trabalho, sem variáveis de ambiente adicionais. `id` identifica o adaptador como `codex`.

## Importações
- `default`, `new`: Módulos do adaptador Codex.
- `crate::harness`: Tipos da interface e da invocação do harness.
