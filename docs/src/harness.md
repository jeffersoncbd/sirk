### Resumo

O arquivo define a abstração comum para executar diferentes coding-agent harnesses. Ele padroniza as requisições de execução, a construção da chamada externa e a conversão da saída do processo em uma resposta textual.

### Funcionamento

`RunRequest` reúne os dados necessários para uma execução: prompt, diretório de trabalho, modelo opcional e indicação de fluxo de eventos.

A trait `HarnessAdapter` representa a fronteira de integração com uma CLI de agente. Cada implementação deve:

- Informar um identificador estático por meio de `id`.
- Transformar um `RunRequest` em uma `Invocation`, podendo retornar `HarnessError`.
- Opcionalmente converter o `stdout` do processo em uma resposta específica. Por padrão, a saída é retornada sem transformação.

`HarnessError` modela falhas de configuração e processamento, incluindo opções não suportadas, modelo ausente, resposta inválida e configuração inválida. A implementação de `Display` produz mensagens legíveis, e a implementação de `std::error::Error` permite seu uso no ecossistema padrão de tratamento de erros do Rust.

Não há execução de processos, concorrência, persistência ou uso de `unsafe` neste arquivo.

### Componentes principais

- `RunRequest`: `struct` pública e imutável por convenção, contendo os parâmetros de uma execução.
- `HarnessAdapter`: `trait` pública que define o contrato para adaptadores de diferentes CLIs.
- `HarnessError`: `enum` pública com as categorias de erro relacionadas aos adaptadores.
- `Invocation`: tipo reexportado publicamente de `crate::services`, permitindo que a interface de harness exponha esse tipo como parte de seu contrato.
- `fmt::Display` para `HarnessError`: formata cada variante em uma mensagem específica.
- `std::error::Error` para `HarnessError`: integra o erro aos mecanismos padrão de erro do Rust.

### integrações

O arquivo depende do módulo interno `crate::services`, de onde importa e reexporta `Invocation`. Também usa `std::fmt` para formatação de erros e `std::path::PathBuf` para representar diretórios de trabalho.

As principais partes públicas são `RunRequest`, `HarnessAdapter`, `HarnessError` e o reexport `Invocation`.
