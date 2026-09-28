## Resumo
Conduz uma conversa com o agente, processando respostas e ferramentas até obter uma resposta final.

## Funcionamento
Registra e salva a entrada; se houver uma pergunta pendente, salva-a e retorna erro. Resolve o adaptador, executa chamadas e salva cada resposta. Trata pedidos de edição, exclusão, leitura ou árvore conforme as permissões configuradas; pedidos de entrada do usuário e respostas vazias retornam erro. Erros de execução e gravação são propagados.

## Importações
- `adapters`: resolve o adaptador e interpreta respostas.
- `RunRequest`: reúne os parâmetros enviados ao agente.
- `Block`, `History`: armazenam e persistem o histórico.
- `Invocation`: representa a chamada ao agente.
- `tools`: identifica e executa ferramentas.
