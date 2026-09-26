## Resumo
Traduz requisições neutras de provider em invocações da CLI do Ollama (`ollama run <modelo> <prompt>`).

## Funcionamento
`OllamaAdapter` implementa `HarnessAdapter` e carrega apenas o nome do executável. O método `invocation` valida o `RunRequest`: exige `model` (caso contrário retorna `HarnessError::MissingModel`) e rejeita `event_stream` com `HarnessError::UnsupportedOption`. Se válido, monta um `Invocation` com o programa clonado, os argumentos `run`/modelo/prompt, o `working_directory` da requisição e ambiente vazio. Nenhum efeito colateral: apenas construção de dados.

## Importações
- `crate::harness::HarnessAdapter`: trait que define `id` e `invocation` para adaptadores.
- `crate::harness::HarnessError`: erros de validação (modelo ausente, opção não suportada).
- `crate::harness::Invocation`: estrutura do comando final a executar.
- `crate::harness::RunRequest`: dados de entrada (prompt, modelo, diretório, event_stream).
