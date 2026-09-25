### Resumo

Este arquivo define a interface comum para executar diferentes coding-agent harnesses. Ele representa uma solicitação de execução, padroniza os adaptadores de integração e fornece um erro específico para opções não suportadas.

### Funcionamento

`RunRequest` reúne os dados necessários para uma execução: prompt, diretório de trabalho, modelo opcional e indicação de uso de fluxo de eventos.

Cada integração concreta deve implementar `HarnessAdapter`, que:

- Retorna um identificador estático por meio de `id()`.
- Converte um `RunRequest` em uma `Invocation`, retornando `HarnessError` quando a configuração não é suportada.

O enum `HarnessError` atualmente representa apenas a tentativa de usar uma opção incompatível com determinado adaptador. Ele implementa `Display` para gerar uma mensagem legível e `std::error::Error` para integração com o ecossistema padrão de erros do Rust.

### Componentes principais

- `RunRequest`: struct pública com os parâmetros de uma execução.
- `HarnessAdapter`: trait pública que define o contrato dos adaptadores de harness.
- `HarnessError`: enum público para erros de configuração ou compatibilidade.
- `Invocation`: reexportado publicamente a partir de `crate::services`, permitindo que consumidores usem esse tipo por meio deste módulo.
- `fmt::Display`: usado para formatar `HarnessError`.

### Dependências e integrações

- Usa `std::path::PathBuf` para representar o diretório de trabalho.
- Usa `std::fmt` para implementar a exibição dos erros.
- Integra-se ao módulo interno `crate::services` por meio do tipo `Invocation`.
- Serve como fronteira entre o restante da aplicação e implementações específicas de CLIs de coding agents.

### Observações

O arquivo apenas define contratos e tipos de dados; não executa processos nem implementa adaptadores concretos. O comportamento de criação da `Invocation` depende das implementações de `HarnessAdapter` e do tipo `Invocation`, cujo conteúdo não foi fornecido.
