## Resumo
Converte requisições neutras de provedor em invocações da CLI do OpenCode.

## Funcionamento
`OpenCodeAdapter` guarda o caminho do executável e implementa `HarnessAdapter`. O método `invocation` monta a linha de comando partindo de `run`, sempre acrescentando `--title new-harness` para evitar uma requisição extra ao modelo na geração do título e deliberadamente omitindo `--auto` para não aprovar permissões automaticamente. `--format json` só entra quando `request.event_stream` é verdadeiro e `--model` apenas quando há modelo definido. O prompt é isolado por `--` no final, evitando interpretação como flag, e o `Invocation` retornado copia o diretório de trabalho com ambiente vazio. Não há `Err(HarnessError)` neste caminho: a falha ocorre apenas na execução do processo.

## Importações
- `mod default` / `mod new`: Fornecem as implementações de `Default` e `new` do adaptador.
- `crate::harness::HarnessAdapter`: Trait que define o contrato `id`/`invocation` para adaptadores.
- `crate::harness::HarnessError`: Tipo de erro do trait, hoje não retornado pela tradução.
- `crate::harness::Invocation`: Estrutura preenchida com programa, argumentos, diretório e ambiente.
- `crate::harness::RunRequest`: Entrada neutra com prompt, working directory, modelo e flag de stream.
