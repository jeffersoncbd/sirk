## Resumo
Módulo de testes do runner que valida a edição por coordenadas, ferramentas do agente e retomada de execuções.

## Funcionamento
Sem código de produção: apenas `#[test]`s. Verificam que `READ` enumerado preserva a versão bruta usada pelo `EDIT` via `version-output`, que o tool `EDIT` do agente é aplicado uma única vez (duplicatas e falhas de intervalo retornam `DUPLICATE_EDIT_RESULT`/`EDIT_FAILURE_PREFIX` para correção), que `ASK`/`AWAIT` exigem entrada e falham em modo não interativo, que `READ` rejeita diretórios, binários e symlinks externos, que `custom-tool` repassa argumentos e diretório de trabalho, e que `LOOP` com templates em escopo grava `version`/`Block` no histórico. Efeitos colaterais observados: escrita/remoção de arquivos e abertura de `History` para `continue_with`, que reconstrói as saídas sem reexecutar passos concluídos. Erros são `Result` propagados dos `unwrap`/`is_err`.

## Importações
- `super::*`: módulo pai do runner (Project, run_with, continue_with, History, Block e constantes de prompt).
