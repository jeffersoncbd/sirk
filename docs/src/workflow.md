## Resumo
Valida estaticamente os passos de um workflow, garantindo conformidade de campos, escopo e referências.

## Funcionamento
Percorre cada `Step` do slice, inferindo o tipo (agente, ferramenta, custom-tool) e rejeitando campos restritos a outros tipos (`is_true`/`is_false` só em IF, `iter` só em LOOP, `version_output`/`enumerate` só em READ, `path`/`force`/`skip` conforme a ferramenta, `operation`/`line`/`start`/`end`/`version` só em EDIT). Valida entradas específicas por ferramenta (input vazio em TREE/GIT-STATUS-TREE/AWAIT/DELETE, pergunta não vazia em ASK, caminho em READ/WRITE/EDIT/DELETE, lista de strings em custom-tool) e constrói um `EditRequest` com valores placeholder para conferir coordenadas e versão. Depois renderiza `input` e `path` contra `outputs`/`locals` para detectar referências inválidas. Aninhamentos são validados recursivamente: LOOP cria escopo com `item`; IF mescla saídas apenas quando presentes em ambas as branches e registra todos os nomes declarados. Por fim, registra `output`/`version_output` em `outputs`, `loop.*` em `locals` ou `declared`, rejeitando duplicatas e `item`. Erros são `Result<(), String>` com mensagens por índice do passo.

## Importações
- `BTreeMap`: Armazena saídas globais e variáveis locais do escopo de loop.
- `BTreeSet`: Rastreia nomes globais já declarados para evitar redeclaração.
- `EditCoordinate`: Modelo de coordenadas usado na verificação de edições.
- `Step`: Tipo do passo validado, com campos de ferramenta e saída.
- `StepInput`: Variante de entrada (texto, booleano, array) validada por ferramenta.
- `Workflow`: Tipo raiz que expõe `validate` e detém a lista de passos.
- `condition`: Avalia expressões booleanas usadas na condição do IF.
- `loop_items`: Confere que o input do LOOP é uma lista JSON de strings.
- `loop_target`: Identifica referências `loop.*` e extrai o nome da chave.
- `render_scoped`: Renderiza templates respeitando o escopo de loop e globais.
- `output_name::valid`: Valida a sintaxe dos nomes de output declarados.
