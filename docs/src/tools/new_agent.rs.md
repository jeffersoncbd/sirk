## Resumo
Cria um arquivo de definição de agente a partir das respostas do usuário e do conteúdo gerado por um adaptador.

## Funcionamento
Resolve e valida o diretório, coleta descrição, adaptadores, modelos e nome do agente, repetindo a pergunta se o nome for inválido ou já existir. Gera a definição, valida seu formato e os metadados solicitados; em caso de erro, retorna `Err` sem salvar. Se válida, cria `.agents` quando necessário e grava o arquivo sem sobrescrever outro existente, retornando seu caminho.

## Importações
- `adapters`: resolve o adaptador gerador.
- `Agent`, `valid_id`: valida a definição e o nome do agente.
- `RunRequest`: prepara a solicitação de geração.
- `UserInput`: coleta respostas do usuário.
- `Invocation`: representa a chamada ao adaptador.
- `Serialize`: serializa os metadados YAML.
- `std::fs`, `OpenOptions`: cria diretórios e arquivos sem sobrescrita.
- `std::io::Write`: grava e sincroniza o conteúdo.
- `Path`, `PathBuf`: manipula diretórios e retorna o caminho criado.
