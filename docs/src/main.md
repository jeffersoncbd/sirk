### Resumo

Este arquivo é o ponto de entrada da aplicação CLI `new-harness`. Ele interpreta argumentos, inicia workflows, retoma históricos, cria agentes interativamente e oferece um modo de operação interativo quando nenhum argumento é informado.

### Funcionamento

A função `main` coleta os argumentos da linha de comando e chama `run`. Em caso de sucesso, retorna `ExitCode::SUCCESS`; em caso de erro, imprime a mensagem em `stderr` e retorna código de saída `2`.

Os comandos suportados são:

- execução interativa sem argumentos;
- criação de agente com `--newAgent` ou `--new-agent`;
- execução de workflow com `run <workflow.yml>`;
- retomada com `resume <history.log>`;
- exibição da ajuda.

No modo interativo, o programa lê comandos linha a linha. Ele aceita:

- `run <workflow.yml>`;
- `resume <history.log>`;
- `/help`;
- `/quit` ou `/exit`.

Erros ocorridos durante a execução interativa são exibidos, mas não encerram o loop. A entrada padrão é encerrada com EOF, o programa termina normalmente.

### Componentes principais

- `main()`: ponto de entrada e conversão do resultado em `ExitCode`.
- `run(arguments)`: interpreta os argumentos da CLI e direciona para a operação correspondente.
- `interactive()`: executa o prompt interativo para workflows.
- `run_workflow(path)`: carrega um `Workflow`, resolve o diretório de trabalho atual e inicia sua execução com `runner::run`.
- `resume_workflow(path)`: retoma uma execução usando `runner::resume`.
- `print_usage()`: imprime a mensagem de ajuda.
- `usage()`: retorna a string estática com os comandos disponíveis.
- `tests::rejects_an_invalid_command()`: verifica que comandos desconhecidos produzem erro.

### Dependências e integrações

O arquivo utiliza módulos da biblioteca padrão:

- `std::env` para argumentos e diretório atual;
- `std::io` para leitura do terminal e flush do prompt;
- `std::path::Path` para manipulação de caminhos;
- `std::process::ExitCode` para códigos de saída.

Também integra módulos internos da crate:

- `new_harness::workflow::Workflow` para carregar workflows a partir de arquivos;
- `new_harness::runner` para executar e retomar workflows;
- `new_harness::tools::new_agent::create` para criação interativa de agentes;
- `new_harness::input::TerminalInput` como fonte de entrada do terminal.

### Observações

As funções deste arquivo são privadas; a interface pública utilizada está nos módulos da crate `new_harness`.

Os erros são propagados com `Result` e `?`, sem uso de `panic!`. A execução em modo não interativo termina com código `2` quando há erro, enquanto o modo interativo apenas informa o erro e continua.

A mensagem de uso menciona que um caminho de workflow pode ser fornecido diretamente, mas a função `run` não possui um caso explícito para um único argumento que seja um caminho; esse comportamento depende apenas do código apresentado.
