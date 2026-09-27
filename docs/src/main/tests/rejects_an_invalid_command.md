## Resumo
Verifica que um comando inválido é rejeitado por `run`.

## Funcionamento
Chama `run` com o argumento `"invalid"`, espera um erro e confirma que a mensagem contém `"invalid command"`.

## Importações
- `super`: importa o contexto pai, incluindo `run`.
