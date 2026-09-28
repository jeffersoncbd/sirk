## Resumo
Verifica que as operações de anexar e antepor produzem o texto esperado e que conflitos de versão são detectados.

## Funcionamento
Testa os resultados de `Append` e `Prepend`, inclusive com conteúdo vazio. Também confirma que uma versão incompatível gera erro e que uma edição sem versão não é aceita nesse caso.

## Importações
- `super::*`: Importa os tipos e auxiliares do módulo pai usados no teste.
