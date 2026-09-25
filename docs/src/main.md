### Resumo

`src/main.rs` é o ponto de entrada da aplicação. Ele interpreta argumentos da linha de comando, oferece um modo interativo e encaminha a execução ou retomada de workflows para os módulos responsáveis.

### Funcionamento

A função `main` chama `run` e converte o resultado em um `ExitCode`: sucesso retorna código zero; erros são exibidos em `stderr` e resultam no código `2`.

Sem argumentos, `run` inicia uma sessão interativa. Também reconhece:

- criação interativa de um agente;
- execução de um workflow a partir de um arquivo;
- retomada de um histórico;
- comandos de ajuda.

No modo interativo, o programa lê linhas do terminal em um loop. Comandos vazios são ignorados, `/quit` e `/exit` encerram a sessão, `/help` mostra a ajuda, e os comandos `run` e `resume` acionam as operações correspondentes. Erros durante a sessão são exibidos sem encerrar imediatamente o processo.

Para executar um workflow, o arquivo é carregado por `Workflow::from_file`, o diretório atual é resolvido e canonicalizado, e a execução é delegada a `runner::run`. A retomada é delegada a `runner::resume`.

Os erros são propagados como `Result<(), String>` usando `?`. Não há uso de `unsafe`, concorrência ou persistência implementada diretamente neste arquivo.

### Componentes principais

- `main`: ponto de entrada e conversão do resultado em código de saída.
- `run`: interpreta os argumentos recebidos e seleciona a operação correspondente.
- `interactive`: implementa o loop de interação pelo terminal.
- `run_workflow`: carrega um workflow, resolve o diretório de trabalho e inicia sua execução.
- `resume_workflow`: solicita ao runner a retomada de um histórico.
- `print_usage`: imprime o texto de ajuda.
- `usage`: fornece a descrição estática dos comandos disponíveis.
- `tests`: contém testes unitários para rejeição de comandos inválidos e consistência do texto de ajuda.

### integrações

- `new_harness::runner`: executa e retoma workflows.
- `new_harness::workflow::Workflow`: carrega e representa um workflow a partir de arquivo.
- `new_harness::tools::new_agent`: cria agentes de forma interativa.
- `new_harness::input::TerminalInput`: fornece a entrada do terminal durante a criação do agente.
- `std::env`, `std::io`, `std::path` e `std::process`: fornecem acesso ao ambiente, terminal, caminhos e códigos de saída.

As funções auxiliares deste arquivo são privadas. A integração externa ocorre principalmente por meio da função pública `main` e das APIs dos módulos `runner`, `Workflow`, `new_agent` e `TerminalInput`, cuja implementação não está presente no conteúdo analisado.
