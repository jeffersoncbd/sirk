## Resumo
Verifica se `{{ outputs.plan }}` é preservado como texto literal na entrada enviada ao agente.

## Funcionamento
Cria um agente temporário com um adaptador que aceita a expressão literal, envia a entrada pela rota HTTP e confirma uma resposta `200` com o resultado esperado. Ao final, remove o diretório temporário.

## Importações
- `handle`: Executa a requisição HTTP testada.
- `std::fs`: Cria arquivos e diretórios temporários.
- `PermissionsExt`: Define permissão executável para o adaptador.
- `SystemTime`, `UNIX_EPOCH`: Geram um nome temporário único.
- `serde_json`: Monta a requisição e interpreta a resposta.
