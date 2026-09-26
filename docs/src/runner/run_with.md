## Resumo
Executa um `Workflow` em modo não interativo, delegando a `run_interactive_with` com um provedor de entrada que nunca solicita dados ao usuário.

## Funcionamento
A função é um adaptador fino: valida nada e não contém regras próprias, apenas encaminha `workflow`, `directory` e o closure `execute` para `run_interactive_with`, substituindo a camada de interação por `&mut NoInput`. O retorno é propagado semTransformation: um `Ok` traz um `BTreeMap<String, String>` com os resultados-chave do fluxo, e um `Err(String)` carrega a mensagem de falha (ex.: erro do `execute` ou do próprio workflow). Sem efeitos colaterais próprios além dos问答 de `run_interactive_with`.

## Importações
- `Invocation`: estrutura de cada invocação passada ao closure `execute`.
- `Workflow`: definição do fluxo a ser executado.
- `BTreeMap`: tipo do mapa de resultados retornado em `Ok`.
- `Path`: diretório de trabalho recebido e repassado.
- `run_interactive_with`: implementação real do fluxo, com suporte a entrada do usuário.
- `NoInput`: provedor de respostas fixas que desabilita prompts interativos.
