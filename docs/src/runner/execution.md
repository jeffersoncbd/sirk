## Resumo

`src/runner/execution.rs` implementa o motor de execução de workflows. Ele percorre os `Step`s definidos, mantém o histórico persistido da execução, executa ferramentas, solicita entradas ao usuário e conversa com agentes por meio de adapters.

## Funcionamento

`continue_with` valida o snapshot e os blocos do histórico, copia os passos do workflow e inicia um `Engine`. O engine mantém:

- uma referência mutável ao `History`;
- a função de execução de invocações externas;
- a interface `UserInput`;
- um cursor global para associar cada passo à sua posição no histórico.

`run_steps` percorre os passos sequencialmente e trata estruturas de controle:

- `LOOP`: avalia a coleção de itens, cria um escopo local com `item` e executa os passos internos para cada elemento;
- `IF`: avalia uma condição, escolhe a ramificação correspondente e executa seu corpo;
- passos comuns: delega a execução a `run_step`, calcula saídas normais e versões enumeradas e armazena esses valores em `outputs` ou no escopo local.

Cada alteração relevante no histórico é salva imediatamente. Isso permite retomar execuções parcialmente concluídas.

`run_step` trata diferentes tipos de passo:

- `EDIT`: prepara e aplica uma alteração, registra o diff e o exibe;
- `AWAIT`: aguarda confirmação do usuário;
- `ASK`: apresenta uma pergunta e registra a resposta;
- ferramentas diretas: executa `WRITE`, `DELETE`, `READ` e ferramentas customizadas;
- passos de agente: constroem um prompt com instruções e histórico da conversa, invocam o adapter correspondente e processam respostas, chamadas de ferramentas, perguntas e solicitações externas de edição ou exclusão.

As respostas dos agentes podem gerar solicitações de `TREE`, `READ`, edição, exclusão ou perguntas adicionais. O engine executa ou encaminha essas operações, adiciona os resultados ao histórico e continua o loop até obter uma resposta final.

Erros são propagados com `Result` e `?`. Também há erros explícitos para histórico inválido, perguntas vazias, ferramentas sem permissão e respostas vazias de agentes.

## Componentes principais

- `continue_with`: ponto de entrada público para continuar um workflow e retornar as saídas finais em um `BTreeMap`.
- `Engine`: struct interna que coordena execução, histórico, entrada do usuário e cursor.
- `run_steps`: percorre passos, controla `LOOP` e `IF` e distribui resultados entre saídas globais e locais.
- `run_step`: executa passos de edição, confirmação, pergunta, ferramentas diretas e agentes.
- `History` e `Block`: persistem entradas, respostas, perguntas e resultados de ferramentas.
- `Step`: fornece configuração, renderização de entradas, caminhos, saídas e ramificações.
- `Invocation`: representa a execução preparada para um adapter.
- `UserInput`: abstrai perguntas e confirmações ao usuário.
- `adapters::resolve`: seleciona o adapter do agente e transforma a requisição em uma invocação.
- `crate::tools`: fornece execução de ferramentas, leitura enumerada, edição, escrita, exclusão e ferramentas customizadas.
- `external::*`: interpreta e acompanha solicitações externas de edição e exclusão feitas pelos agentes.
- `condition`, `loop_items` e `loop_target`: validam estruturas de controle e identificam saídas destinadas a escopos de loop.

## integrações

O arquivo expõe publicamente `continue_with`, que recebe um `History`, uma função de execução baseada em `Invocation` e uma implementação de `UserInput`.

Internamente, integra-se com:

- o sistema de histórico e persistência do workflow;
- adapters de agentes e `RunRequest`;
- ferramentas de leitura, escrita, edição, exclusão e execução;
- entrada interativa do usuário;
- estruturas de workflow, condições, loops e saídas;
- serialização JSON para estados pendentes de edição e chamadas externas.
