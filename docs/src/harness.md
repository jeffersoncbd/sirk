### Resumo

Este arquivo define a interface comum para executar diferentes coding-agent harnesses por linha de comando. Ele padroniza a solicitação de execução, a criação da invocação do processo e os erros específicos de integração.

### Funcionamento

`RunRequest` reúne os dados necessários para uma execução: prompt, diretório de trabalho, modelo opcional e indicação de saída por fluxo de eventos.

A trait pública `HarnessAdapter` representa a integração com um CLI específico. Cada adapter deve:

- informar seu identificador por meio de `id`;
- transformar um `RunRequest` em uma `Invocation`, ou retornar um `HarnessError`.

`HarnessError` modela duas falhas de configuração:

- `UnsupportedOption`: o adapter não suporta determinada opção;
- `MissingModel`: o adapter exige um modelo, mas ele não foi fornecido.

A implementação de `Display` gera mensagens legíveis para esses erros, e a implementação de `std::error::Error` permite usá-los no ecossistema padrão de tratamento de erros do Rust.

### Componentes principais

- `RunRequest`: struct pública que representa uma solicitação independente do provedor.
- `HarnessAdapter`: trait pública que define o contrato de integração com um harness.
- `HarnessError`: enum público com erros de validação ou configuração.
- `Invocation`: tipo reexportado publicamente de `crate::services`, usado para representar a execução preparada do processo.
- `fmt::Display`: usado para formatar mensagens de erro.

### Dependências e integrações

- `std::fmt`: utilizado na implementação de `Display`.
- `std::path::PathBuf`: representa o diretório de trabalho da execução.
- `crate::services::Invocation`: conecta este contrato à camada responsável por executar processos.
- O restante do projeto deve fornecer implementações concretas de `HarnessAdapter` para cada CLI suportado.

### Observações

O arquivo apenas define contratos e dados; não executa processos, não realiza operações assíncronas ou concorrentes e não contém persistência. A lógica concreta de montagem dos argumentos fica nas implementações da trait `HarnessAdapter`.
