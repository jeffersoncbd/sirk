### Resumo

Este arquivo implementa a criação interativa de definições de agentes em `.agents/<nome>.md`. Ele coleta os dados do usuário, solicita a outro adapter/modelo que gere as instruções em Markdown, valida o resultado e grava o arquivo apenas se os metadados gerados forem compatíveis com os escolhidos.

### Funcionamento

O fluxo principal ocorre em `create_with`:

1. Canonicaliza o diretório do projeto.
2. Solicita:
   - descrição do agente;
   - adapter que o agente criado usará;
   - modelo do agente;
   - adapter e modelo responsáveis por gerar a definição;
   - nome válido e ainda não utilizado.
3. Monta um cabeçalho YAML com `adapter` e `model`.
4. Cria um prompt para o agente gerador, exigindo:
   - apenas o conteúdo do arquivo;
   - front matter YAML;
   - instruções detalhadas;
   - preservação do idioma usado pelo usuário;
   - ausência de metadados não suportados.
5. Resolve o adapter gerador e cria uma `Invocation`.
6. Executa a geração por meio da função recebida em `generate`.
7. Analisa o resultado com `Agent::parse`.
8. Verifica se o agente gerado:
   - manteve o adapter e modelo solicitados;
   - não habilitou `json`;
   - não incluiu configuração de interação `ask`.
9. Recria o front matter usando os metadados selecionados pelo usuário e grava as instruções geradas em um novo arquivo.

A função pública `create` fornece a execução real usando `BashService::execute_streaming`. Já `create_with` permite injetar uma função geradora, facilitando testes sem executar um modelo real.

### Componentes principais

- `create`: ponto de entrada para a criação real. Executa a `Invocation` por meio de `BashService` e rejeita processos que terminem com status de erro.

- `create_with`: contém todo o fluxo de entrada, geração, validação e persistência. Recebe uma função `generate` para abstrair a execução do gerador.

- `choose_adapter`: apresenta os adapters disponíveis em `adapters::AVAILABLE`, normaliza a entrada para minúsculas e repete a pergunta até receber um adapter suportado.

- `nonempty`: solicita uma resposta até que ela não esteja vazia.

- `Header`: struct local derivada de `Serialize`, usada para produzir o front matter YAML com os campos `adapter` e `model`.

- `valid_id`: função importada de `agents`, usada para validar o nome do arquivo do agente.

- `Agent::parse`: interpreta e valida o conteúdo Markdown retornado pelo gerador.

- `Answers`: implementação de teste de `UserInput` baseada em `VecDeque`, fornecendo respostas predeterminadas.

- `Project`: fixture de testes que cria um diretório temporário e o remove ao final.

### Dependências e integrações

- `crate::adapters`: resolve adapters e lista os adapters disponíveis.
- `crate::agents::{Agent, valid_id}`: valida nomes e interpreta definições de agentes.
- `crate::harness::RunRequest`: transporta prompt, diretório de trabalho, modelo e configuração de eventos.
- `crate::input::UserInput`: abstrai a entrada interativa do usuário.
- `crate::services::{BashService, Invocation}`: executa o processo externo e representa sua invocação.
- `serde::Serialize` e `serde_yaml`: serializam o front matter YAML.
- `std::fs` e `OpenOptions`: criam diretórios e arquivos.
- `Path` e `PathBuf`: manipulam caminhos do sistema de arquivos.

### Observações

- O arquivo é explicitamente destinado à criação independente de agentes e não está disponível para workflows ou solicitações de modelos.
- Erros são propagados como `Result<_, String>`, incluindo falhas de entrada, resolução de adapter, geração, parsing e gravação.
- O arquivo de destino é aberto com `create_new(true)`, evitando sobrescrever um agente existente.
- O diretório `.agents` só é criado depois que a geração e a validação terminam com sucesso.
- Os testes verificam criação, preservação do idioma do prompt, cancelamento, colisões de nomes e rejeição de definições inválidas.
- O comportamento de `Agent::parse`, de `adapters::resolve` e da execução de `BashService` depende de outros módulos não incluídos no conteúdo fornecido.
