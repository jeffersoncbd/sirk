### Resumo

O arquivo `src/tools/new_agent.rs` implementa a criação interativa de arquivos de definição de agentes em `.agents/<nome>.md`. Ele coleta metadados do usuário, solicita a outro adapter a geração das instruções, valida o resultado e grava o arquivo somente se a definição for válida.

### Funcionamento

O fluxo principal ocorre em `create_with`:

1. Canonicaliza o diretório do projeto.
2. Solicita uma descrição não vazia para o novo agente.
3. Valida os adapters e modelos do agente criado e do agente gerador.
4. Solicita um nome válido, rejeitando nomes inválidos ou já existentes.
5. Monta um front matter YAML contendo `adapter` e `model`.
6. Cria um prompt para o adapter gerador, exigindo apenas o conteúdo Markdown do agente.
7. Resolve o adapter escolhido e cria uma `Invocation` com o prompt, diretório de trabalho, modelo e sem fluxo de eventos.
8. Executa a geração por meio de uma função injetada ou, em `create`, pelo `BashService`.
9. Analisa o texto retornado com `Agent::parse`.
10. Verifica se o gerador preservou o adapter e modelo solicitados e se não adicionou metadados proibidos como `json` ou `ask`.
11. Cria `.agents`, grava o arquivo com `create_new(true)` e chama `sync_all`.

Erros são propagados como `Result<_, String>`. Falhas de entrada, geração, parsing, validação ou escrita impedem a criação do arquivo. O uso de `create_new(true)` também evita sobrescrever arquivos existentes.

### Componentes principais

- `create`: ponto de entrada que usa `BashService::execute_streaming` para executar a geração real e rejeita processos encerrados com status de erro.
- `create_with`: núcleo testável da criação de agentes; recebe uma função de geração injetável.
- `choose_adapter`: apresenta os adapters disponíveis e repete a pergunta até receber um valor aceito.
- `nonempty`: exige uma resposta não vazia.
- `Header`: struct privada serializada para YAML, contendo `adapter` e `model`.
- `Agent::parse`: valida e interpreta a definição Markdown retornada pelo gerador.
- `BashService` e `Invocation`: abstraem a execução externa do adapter gerador.
- `RunRequest`: transporta prompt, diretório, modelo e configuração de eventos para a criação da invocação.
- `UserInput`: fornece a interface de perguntas interativas.
- Módulo de testes: usa respostas simuladas, diretórios temporários e callbacks falsos para verificar criação, cancelamento, colisões de nomes e rejeição de definições inválidas.

### integrações

As funções públicas expostas são:

- `create(directory, input) -> Result<PathBuf, String>`
- `create_with(directory, input, generate) -> Result<PathBuf, String>`

O arquivo integra-se com:

- `crate::adapters`, para listar e resolver adapters disponíveis.
- `crate::agents`, para validar nomes e interpretar definições de agentes.
- `crate::harness`, para construir requisições de execução.
- `crate::input`, para coletar entradas.
- `crate::services`, para executar o processo externo e representar sua invocação.
- `serde`, para serializar o front matter YAML.
- Sistema de arquivos local, para criar e persistir arquivos `.agents/*.md`.
