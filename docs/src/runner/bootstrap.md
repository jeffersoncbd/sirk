## Resumo

O arquivo `src/runner/bootstrap.rs` inicializa e valida execuções de workflows. Ele prepara agentes, resolve o diretório de trabalho, cria ou abre o histórico da execução e delega o processamento à lógica de continuação em `execution::continue_with`.

## Funcionamento

O fluxo principal é:

1. Validar o `Workflow`.
2. Canonicalizar o diretório de execução.
3. Percorrer recursivamente todas as etapas, incluindo etapas iterativas e condicionais.
4. Carregar, sem duplicação, as configurações dos agentes referenciados em `.agents`.
5. Criar um `Snapshot` contendo diretório, workflow e agentes.
6. Validar esse snapshot.
7. Criar ou abrir o histórico e continuar a execução.

A função interna `execute` executa uma `Invocation` por meio de `BashService` em modo streaming. Erros de execução são convertidos para `String`, e processos encerrados com status diferente de sucesso produzem erro sem considerar a saída incompleta como resultado válido.

Há dois modos de entrada:

- `run` usa entrada interativa pelo terminal.
- `run_with` usa uma implementação `NoInput`, adequada para testes ou execução não interativa; qualquer solicitação de entrada resulta em erro.
- `run_interactive_with` permite fornecer uma implementação personalizada de `UserInput`.

A validação do snapshot confirma que:

- O workflow continua válido.
- O diretório ainda existe.
- Cada agente referenciado possui configuração carregada.
- O adapter do agente é conhecido.
- Agentes que produzem streams JSON não são usados em conversas retomáveis.
- O campo `ask`, quando presente, não contém apenas espaços.

A função `question` reconhece mensagens iniciadas por `ASK:` após espaços iniciais e retorna o texto da pergunta.

## Componentes principais

- `execute`: executa comandos através de `BashService` e retorna a saída padrão somente quando o processo termina com sucesso.
- `run`: inicia uma execução com entrada interativa do terminal.
- `resume`: abre um histórico existente e retoma a execução.
- `run_with`: inicia uma execução sem entrada interativa, usando um executor injetado.
- `run_interactive_with`: prepara e inicia uma execução com executor e entrada fornecidos pelo chamador.
- `validate_snapshot`: valida o estado persistido antes de continuar uma execução.
- `question`: extrai perguntas prefixadas por `ASK:`.
- `all_steps`: percorre recursivamente etapas normais, iterativas e condicionais.
- `NoInput`: implementação privada de `UserInput` que rejeita solicitações de interação.
- `History`, `Snapshot`, `Workflow` e `Step`: representam o histórico, o estado inicial da execução e a estrutura do workflow.
- `Agent` e `adapters::resolve`: carregam configurações de agentes e verificam adapters disponíveis.
- `BashService` e `Invocation`: abstraem a execução de processos externos.
- `TerminalInput` e `UserInput`: fornecem entrada interativa ou substituível.

## integrações

O arquivo expõe publicamente:

- `run`
- `resume`
- `run_with`
- `run_interactive_with`

Também disponibiliza internamente ao módulo pai:

- `validate_snapshot`
- `question`

A execução depende de módulos internos de agentes, adapters, histórico, entrada, serviços e workflows, além da execução posterior em `super::execution::continue_with`.
