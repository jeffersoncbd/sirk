### Resumo

O arquivo `src/runner.rs` funciona como a fachada pública do módulo de execução de workflows. Ele mantém a API estável da crate enquanto delega a implementação do motor de execução e recuperação ao submódulo privado `runner/engine.rs`.

### Funcionamento

O módulo declara `engine` como um submódulo privado, isolando os detalhes internos do motor. Em seguida, reexporta seletivamente as funções públicas de execução e retomada definidas em `engine`, permitindo que outras partes do projeto acessem essas operações por meio de `runner` sem depender diretamente da organização interna do motor.

O arquivo não contém lógica própria de execução, validações, persistência, tratamento de entradas ou comunicação externa. Esses comportamentos pertencem a `runner/engine.rs` e aos módulos utilizados por ele.

### Componentes principais

- `mod engine`: declara o submódulo privado que contém o motor stateful de execução e recuperação.
- `run`: função pública reexportada para iniciar uma execução.
- `resume`: função pública reexportada para retomar uma execução existente.
- `run_with`: função pública reexportada para executar um workflow com uma estratégia de entrada ou interação fornecida.
- `run_interactive_with`: função pública reexportada para executar um workflow com interação.
- `continue_with`: função pública reexportada para continuar uma execução a partir de seu estado persistido.

### integrações

O arquivo expõe publicamente `run`, `resume`, `run_with`, `run_interactive_with` e `continue_with`, todos definidos no módulo privado `engine`. A implementação efetiva e suas dependências não aparecem neste arquivo; para compreender o fluxo de execução, recuperação, histórico e tratamento de erros, é necessário analisar `runner/engine.rs` e os módulos que ele importa.