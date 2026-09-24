### Resumo

Este arquivo implementa a criação standalone de um novo agente Markdown em `.agents/`. Ele coleta configurações do usuário, solicita a geração das instruções a outro agente/modelo, valida o resultado e salva a definição do agente sem permitir sobrescrever arquivos existentes.

### Funcionamento

O fluxo principal é:

1. Canonicaliza o diretório do projeto.
2. Solicita:
   - descrição do agente;
   - adapter e modelo que o agente criado usará;
   - adapter e modelo usados para gerar a definição;
   - nome do agente.
3. Valida adapters, campos não vazios, nome e colisões de arquivos.
4. Monta um prompt contendo o front matter YAML esperado e a descrição do usuário.
5. Resolve o adapter gerador e cria uma `Invocation`.
6. Executa a geração por meio de `BashService`.
7. Analisa o Markdown retornado com `Agent::parse`.
8. Confirma que o gerador não alterou o adapter/modelo nem adicionou metadados proibidos.
9. Cria `.agents/<nome>.md` usando `create_new`, preservando agentes existentes.
10. Grava e sincroniza o arquivo em disco.

A função `create` usa a execução real via `BashService`, enquanto `create_with` recebe uma função de geração injetável, facilitando testes sem executar um modelo real.

### Componentes principais

- `create`: ponto de entrada para criação usando o serviço real de execução de processos.
- `create_with`: implementa todo o fluxo de coleta, geração, validação e persistência.
- `choose_adapter`: apresenta os adapters disponíveis e repete a pergunta até receber um valor válido.
- `nonempty`: repete a pergunta enquanto a resposta estiver vazia.
- `Header`: struct local serializada como front matter YAML, contendo `adapter` e `model`.
- `Answers`: implementação de teste de `UserInput`, baseada em uma fila de respostas.
- `Project`: utilitário de testes que cria e remove diretórios temporários.
- Testes:
  - verificam criação e carregamento do agente;
  - preservam o idioma e a descrição original do usuário como entrada do gerador;
  - testam cancelamento e colisões de nomes;
  - garantem que erros ou definições inválidas não criem agentes.

### Dependências e integrações

- `crate::adapters`: lista adapters disponíveis e transforma a configuração em uma `Invocation`.
- `crate::agents::{Agent, valid_id}`: valida nomes e interpreta a definição Markdown gerada.
- `crate::harness::RunRequest`: representa a solicitação enviada ao adapter gerador.
- `crate::input::UserInput`: abstrai a entrada interativa do usuário.
- `crate::services::{BashService, Invocation}`: executa o processo externo e transporta seus argumentos.
- `serde::Serialize` e `serde_yaml`: serializam o front matter YAML.
- `std::fs` e `OpenOptions`: criam diretórios e salvam o arquivo no sistema de arquivos.

### Observações

- O diretório do projeto precisa existir, pois `canonicalize` é executado no início.
- O arquivo final usa sempre o adapter e modelo escolhidos pelo usuário, mesmo que o gerador tenha retornado outra capitalização.
- O gerador deve retornar somente uma definição válida; respostas vazias, metadados alterados ou opções proibidas são rejeitados.
- O uso de `create_new(true)` impede sobrescrever agentes existentes.
- O conteúdo gerado é validado antes de ser salvo, mas a escrita do arquivo ocorre diretamente no destino; uma falha durante a gravação pode deixar um arquivo incompleto.
