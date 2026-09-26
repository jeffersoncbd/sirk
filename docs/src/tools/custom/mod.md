## Resumo
Agrega a ferramenta `custom-tool`, que executa um script Bash local do projeto com argumentos posicionais.

## Funcionamento
Declara os submódulos `args`, `name` e `run` e reexporta `arguments`, `valid_name` e `execute`, expondo a API do comando a partir de um único ponto de importação.

## Importações
- `args`: módulo interno que interpreta os argumentos posicionais.
- `name`: módulo interno que valida o nome da ferramenta.
- `run`: módulo interno que executa o script Bash.
