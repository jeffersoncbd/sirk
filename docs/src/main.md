### Resumo

Este arquivo é o ponto de entrada binário da aplicação `new-harness`. Ele interpreta argumentos da linha de comando, inicia sessões interativas, carrega workflows, executa ou retoma históricos e apresenta erros ao usuário.

### Funcionamento

`main` coleta os argumentos recebidos e chama `run`. Em caso de sucesso, retorna `ExitCode::SUCCESS`; em caso de erro, imprime a mensagem em `stderr` e encerra com código `2`.

O comando é interpretado assim:

- Sem argumentos: inicia o modo interativo.
- `--newAgent` ou `--new-agent`: cria um agente interativamente.
- `run <workflow.yml>`: carrega e executa um workflow.
- `resume <history.log>`: retoma um histórico editável.
- `help`, `--help` ou `-h`: exibe o uso.
- Qualquer outra combinação produz erro de comando inválido.

No modo interativo, o programa exibe um prompt, lê linhas da entrada padrão e aceita:

- `run <workflow.yml>` ou apenas um caminho de workflow;
- `resume <history.log>`;
- `/help`;
- `/quit` e `/exit`.

Erros ocorridos durante comandos interativos são exibidos, mas não encerram o loop. A sessão termina quando a entrada padrão chega ao fim ou quando o usuário usa um comando de saída.

### Componentes principais

- `main()`: converte o resultado da execução em um `ExitCode`.
- `run(arguments)`: faz o despacho dos comandos da CLI.
- `interactive()`: implementa o prompt interativo e processa comandos digitados.
- `run_workflow(path)`: carrega um `Workflow` usando `Workflow::from_file`, resolve o diretório atual e chama `runner::run`.
- `resume_workflow(path)`: chama `runner::resume` para continuar um histórico.
- `print_usage()`: imprime a mensagem de ajuda.
- `usage()`: fornece a string estática com os comandos suportados.
- `tests::rejects_an_invalid_command`: verifica que comandos desconhecidos retornam erro.

### Dependências e integrações

- `std::env`, `std::io`, `std::path` e `std::process` fornecem acesso a argumentos, terminal, caminhos e códigos de saída.
- `new_harness::runner` executa e retoma workflows.
- `new_harness::workflow::Workflow` representa e carrega workflows a partir de arquivos.
- `new_harness::tools::new_agent::create` cria agentes de forma interativa.
- `new_harness::input::TerminalInput` fornece a entrada do terminal usada durante a criação do agente.

### Observações

O tratamento de erros usa `Result<(), String>` e o operador `?`, propagando falhas até `main`. Não há `panic!`, `unsafe`, concorrência ou persistência implementados diretamente neste arquivo.

A mensagem de uso documenta que um caminho de workflow pode ser passado diretamente (`new-harness <workflow.yml>`), mas o despacho não implementa esse formato na linha de comando; ele só é aceito diretamente no modo interativo.
