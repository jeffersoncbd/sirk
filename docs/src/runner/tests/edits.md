## Resumo

O arquivo `src/runner/tests/edits.rs` contém testes de integração do executor de workflows, especialmente para ferramentas de leitura, edição, interação com agentes, recuperação após falhas e retomada por histórico.

## Funcionamento

Os testes criam projetos temporários, arquivos de trabalho e workflows YAML, executando-os por meio de `run_with` ou `run_interactive_with`. Em seguida, verificam:

- leitura numerada e preservação de versões usadas pelo `EDIT`;
- aplicação de edições externas solicitadas por agentes;
- prevenção da repetição de uma edição já concluída;
- retorno de erros de coordenadas para que o agente corrija a solicitação;
- pausa e retomada de etapas `AWAIT` e `ASK`;
- persistência das respostas e dos resultados no histórico;
- recuperação após falhas de leitura ou interrupções;
- rejeição de diretórios, arquivos binários e caminhos fora do projeto;
- passagem segura de argumentos para ferramentas personalizadas;
- uso de `READ` e `EDIT` dentro de loops, incluindo escopos como `loop.item` e `loop.revision`.

Os testes usam `assert!`, `assert_eq!` e `matches!` para comparar saídas, conteúdo de arquivos, histórico e mensagens enviadas aos agentes. Também usam callbacks falsos para simular invocações do harness sem acessar modelos reais.

## Componentes principais

- `#[test]`: cada função é um cenário independente de comportamento do runner.
- `Project::new()`: cria o projeto temporário usado pelos testes.
- `Workflow`: representa workflows carregados a partir de YAML.
- `run_with`: executa workflows não interativos.
- `run_interactive_with`: executa workflows que exigem entrada do usuário.
- `continue_with`: retoma uma execução a partir de um objeto `History`.
- `History`: abre e verifica o histórico persistido da execução.
- `Block`: valida eventos registrados, como perguntas, respostas e resultados de leitura.
- `crate::tools::read::read`: testa diretamente as regras de leitura de arquivos.
- `crate::tools::edit::version`: calcula e valida a versão do conteúdo antes da edição.
- `READ`, `EDIT`, `ASK`, `AWAIT`, `WRITE` e `LOOP`: ferramentas de workflow exercitadas pelos testes.
- `custom-tool`: verifica execução de scripts locais com argumentos tratados como valores literais.
- `DUPLICATE_EDIT_RESULT` e `EDIT_FAILURE_PREFIX`: constantes usadas para validar mensagens de duplicidade e falha de edição.
- `super::*`: importa os elementos do módulo de testes pai, incluindo estruturas auxiliares, funções de execução e utilitários como `answers`.

## integrações

O arquivo não define componentes públicos novos; ele apenas testa APIs e funções importadas do módulo superior e de `crate::tools`.

As principais interfaces verificadas são:

- execução e retomada de workflows;
- persistência e leitura de `History`;
- comunicação simulada com o harness por meio de `Invocation`;
- ferramentas externas `READ`, `EDIT`, `WRITE`, `ASK`, `AWAIT` e `LOOP`;
- execução de scripts em `tools/`;
- leitura e alteração de arquivos dentro dos limites do projeto.

Os testes também demonstram efeitos externos controlados: criação, leitura, alteração e remoção de arquivos temporários, além da persistência de logs de execução.
