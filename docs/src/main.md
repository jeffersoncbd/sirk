### Resumo

`src/main.rs` é o ponto de entrada da aplicação. Ele interpreta os argumentos da linha de comando, cria agentes, executa workflows ou retoma históricos, delegando essas operações aos módulos correspondentes.

### Funcionamento

A função `main` chama `run` e converte o resultado em um `ExitCode`: sucesso retorna código zero; erros são exibidos em `stderr` e resultam no código `2`.

`run` reconhece os seguintes comandos:

- `--newAgent` ou `--new-agent`: cria um agente interativamente no diretório atual, usando `new_agent::create` e `TerminalInput`.
- `run <flow-name>`: valida o nome, resolve o arquivo `flows/<flow-name>.yml`, carrega o workflow e inicia sua execução.
- `resume <history.log>`: solicita ao runner a retomada do histórico indicado.
- `help`, `--help` ou `-h`: imprime a ajuda.
- nenhum argumento: imprime a ajuda.

Os nomes de workflows devem ser não vazios e conter apenas caracteres ASCII alfanuméricos, `_` ou `-`; caminhos, extensões e separadores de diretório são rejeitados. Durante a execução, o diretório atual é canonicalizado antes de ser passado ao runner.

Os erros são propagados como `Result<(), String>` usando `?` e convertidos em mensagens de erro e código de saída pela função `main`. Não há uso de `unsafe`, concorrência ou persistência implementada diretamente neste arquivo.

### Componentes principais

- `main`: ponto de entrada e conversão do resultado em código de saída.
- `run`: interpreta os argumentos e seleciona a operação correspondente.
- `run_workflow`: valida o nome, carrega o workflow e inicia sua execução.
- `workflow_path`: valida nomes e constrói caminhos dentro do diretório `flows`.
- `resume_workflow`: solicita ao runner a retomada de um histórico.
- `print_usage` e `usage`: exibem e fornecem o texto estático de ajuda.
- `tests`: contém testes unitários para comandos inválidos, ajuda e validação de nomes de workflows.

### integrações

- `new_harness::runner`: executa workflows e retoma históricos.
- `new_harness::workflow::Workflow`: carrega um workflow a partir de arquivo.
- `new_harness::tools::new_agent`: cria agentes de forma interativa.
- `new_harness::input::TerminalInput`: fornece a entrada usada na criação do agente.
- `std::env`: acessa argumentos e o diretório atual.
- `std::path`: representa e compõe caminhos de arquivos.
- `std::process::ExitCode`: representa o código de saída do processo.

As funções auxiliares deste arquivo são privadas. O ponto de entrada exposto pelo binário é `main`; as demais integrações públicas pertencem aos módulos importados, cuja implementação não está presente no conteúdo analisado.