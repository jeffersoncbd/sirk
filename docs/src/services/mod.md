### Resumo

Este arquivo define o módulo de serviços relacionados à execução de comandos Bash. Ele expõe tipos públicos para descrever invocações de processos e reexporta as implementações responsáveis pela execução e pelos resultados dos processos.

### Funcionamento

O módulo interno `bash` é declarado com `mod bash`, tornando sua implementação privada neste nível. A estrutura `Invocation` representa uma execução de programa de forma estruturada, separando:

- Programa a executar.
- Lista de argumentos.
- Diretório de trabalho.
- Variáveis de ambiente específicas do processo filho.

As variáveis de ambiente ficam armazenadas em um `BTreeMap` e não devem ser incorporadas ao texto do comando shell, conforme indicado pelo comentário da estrutura.

### Componentes principais

- `BashService`: reexportado de `bash`; representa o serviço de execução de comandos Bash.
- `ProcessOutput`: reexportado de `bash`; representa a saída produzida por um processo.
- `Invocation`: struct pública que descreve um processo a ser executado.
  - `program`: nome ou caminho do executável.
  - `arguments`: argumentos passados ao executável.
  - `working_directory`: diretório de trabalho usando `PathBuf`.
  - `environment`: variáveis de ambiente adicionais usando `BTreeMap<String, String>`.

A struct deriva `Debug`, `Clone`, `PartialEq` e `Eq`, permitindo depuração, cópia e comparação por igualdade.

### integrações

O arquivo expõe publicamente `BashService`, `ProcessOutput` e `Invocation`. A implementação detalhada do módulo `bash` permanece privada, embora seus dois tipos principais sejam disponibilizados por meio de `pub use`.
