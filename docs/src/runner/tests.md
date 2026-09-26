## Resumo
Módulo de fixtures compartilhadas (setup/cleanup) para os testes do runner, sem lógica de produção.

## Funcionamento
Cria o diretório-base `Project` em `temp_dir` com nome único derivado do relógio do sistema, já populando `.agents/planner.md` e `.agents/second.md`; `log()` localiza o arquivo `history/*.log` e `init_git()` executa `git init` via `BashService` (descarando saída em `sink`) e grava `.gitignore`/`visible.txt`/`ignored.txt`. O `Drop` remove a árvore recursivamente, atribuindo falhas a `_` (cleanup nunca propaga erro). `Answers` implementa `UserInput::ask` consumindo um `VecDeque` e retornando `Err("EOF")` quando esgotado, permitindo simular fim de entrada; `workflow()` desserializa um YAML de dois passos encadeando `{{ outputs.plan }}`. Os `mod` com `#[path]` agregam os subconjuntos `control_flow`, `conversations`, `edits` e `files`.

## Importações
- `super::{DUPLICATE_EDIT_RESULT, EDIT_FAILURE_PREFIX, continue_with, run_interactive_with, run_with}`: Constantes e atalhos de execução usados pelos testes.
- `crate::history::{Block, History, Snapshot}`: Tipos do log de histórico inspecionado pelos testes.
- `crate::input::UserInput`: Trait de prompt simulado por `Answers`.
- `crate::services::{BashService, Invocation}`: Execução do `git init` na preparação do fixture.
- `crate::workflow::Workflow`: Tipo desserializado do YAML de exemplo.
- `std::collections::{BTreeMap, VecDeque}`: Fila de respostas e mapa auxiliar de estado.
- `std::fs`: Criação/remoção de arquivos e diretórios temporários.
- `std::path::PathBuf`: Caminhos do projeto temporário.
- `std::time::{SystemTime, UNIX_EPOCH}`: Gera sufixo único de nanossegundos no nome do diretório.
