## Resumo

O arquivo `src/runner/tests.rs` reúne fixtures, utilitários e módulos de testes compartilhados para validar o comportamento do runner, especialmente fluxos de execução, conversas, edições e manipulação de arquivos.

## Funcionamento

Ele cria projetos temporários com agentes de teste definidos em `.agents/`, incluindo um planejador e um revisor. Também fornece:

- Inicialização opcional de um repositório Git de teste.
- Localização do arquivo de histórico gerado.
- Remoção automática do projeto temporário ao final do teste.
- Respostas predeterminadas para simular interações com o usuário.
- Construção de um `Workflow` a partir de YAML, com duas etapas encadeadas pelos outputs dos agentes.

As operações de filesystem e Git usam `unwrap()`, fazendo o teste entrar em panic quando uma preparação falha. A implementação de `UserInput` retorna erro `"EOF"` quando não há mais respostas disponíveis.

## Componentes principais

- `control_flow`, `conversations`, `edits` e `files`: módulos filhos que organizam grupos específicos de testes.
- `Project(PathBuf)`: fixture privada que representa um projeto temporário.
  - `new()`: cria a estrutura `.agents/` e grava as definições dos agentes.
  - `log()`: encontra o arquivo `.log` dentro do diretório `history`.
  - `init_git()`: inicializa um repositório Git e cria arquivos visíveis e ignorados.
  - `Drop`: remove recursivamente o projeto temporário.
- `Answers(VecDeque<String>)`: implementação privada de `UserInput` baseada em uma fila de respostas.
- `answers()`: converte uma lista de strings em `Answers`.
- `workflow()`: desserializa um workflow YAML com as etapas `planner` e `second`.
- Imports de `runner`, `history`, `input`, `services` e `workflow`: fornecem constantes, helpers de execução, tipos de histórico, entrada simulada, execução de comandos e definição de workflow.

## integrações

O arquivo expõe indiretamente seus módulos de teste por meio das declarações `mod`. Os demais elementos (`Project`, `Answers`, `answers` e `workflow`) são privados ao módulo de testes e servem como infraestrutura compartilhada. Ele integra filesystem local, comandos Git via `BashService`, workflows YAML via `serde_yaml` e a trait `UserInput` para simular entrada interativa.
