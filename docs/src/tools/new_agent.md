## Resumo
Cria interativamente um arquivo de agente Markdown em `.agents/` a partir da descrição do usuário, gerando o conteúdo via um adapter LLM.

## Funcionamento
Canonicaliza o diretório do projeto e coleta, via `UserInput`, descrição, adapter/modelo do agente a criar, adapter/modelo gerador e o nome do agente — validando o nome com `valid_id` e re-perguntando em caso de colisão com arquivo existente. Monta um front matter YAML canônico e um prompt que instrui o gerador a devolver apenas o Markdown no idioma do usuário. A resposta é parseada por `Agent::parse` e validada: se o gerador alterar adapter/modelo ou incluir `json`/`ask`, tudo é descartado sem criar arquivo. Só então o arquivo é gravado com `create_new` (sem sobrescrever) após `sync_all`, retornando o `PathBuf`; qualquer erro de geração, parse ou escrita propaga `Err(String)` e não deixa resíduos no disco.

## Importações
- `adapter`: seleção e validação dos adapters (do agente e do gerador)
- `answer`: leitura de respostas não vazias do usuário
- `create`: reexporta o fluxo padrão de criação
- `adapters`: resolve o adapter gerador e monta a `Invocation`
- `Agent`: parse e validação da definição gerada
- `valid_id`: valida o formato do nome do arquivo
- `RunRequest`: parâmetros de execução do gerador
- `UserInput`: abstração de perguntas interativas
- `Invocation`: descrição do processo a ser executado
- `Serialize`: deriva para o front matter YAML
- `fs`: criação do diretório `.agents`
- `OpenOptions`: abertura exclusiva do arquivo do agente
- `Write`: escrita do conteúdo gerado
- `Path`: diretório do projeto
- `PathBuf`: caminho de retorno do arquivo criado
- `serde_yaml`: serialização do front matter
