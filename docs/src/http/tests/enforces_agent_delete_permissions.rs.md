## Resumo
Verifica que a exclusão de arquivos exige permissão explícita do agente.

## Funcionamento
Cria um agente que solicita excluir `note.txt` e executa a requisição sem confirmação e com confirmação permitida. Em ambos os casos, verifica a resposta; o arquivo só deve ser removido quando ambas as permissões estão liberadas.

## Importações
- `handle`: Processa a requisição HTTP do agente.
- `std::fs`: Cria arquivos e verifica sua remoção.
- `PermissionsExt`: Torna o adaptador executável.
- `SystemTime` e `UNIX_EPOCH`: Geram um nome temporário único.
