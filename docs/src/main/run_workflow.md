## Resumo
Carrega um workflow por nome, resolve o diretório de trabalho atual e executa o workflow.

## Funcionamento
A função recebe o nome do workflow e, via `super::workflow_path::workflow_path`, obtém seu caminho; em seguida, desserializa o arquivo com `Workflow::from_file`. Depoiscanonicaliza o diretório atual do processo (mapeando falhas de I/O para `String`) e chama `new_harness::runner::run` passando o workflow e esse diretório. Qualquer erro em qualquer etapa é propagado via `?`, e o retorno bem-sucedido é `Ok(())`.

## Importações
- `std::env`: Fornece `current_dir` para resolver o diretório de trabalho.
