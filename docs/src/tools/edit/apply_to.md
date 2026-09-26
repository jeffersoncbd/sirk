## Resumo
Aplica a operação definida em `Request` ao conteúdo `before`, produzindo uma nova string ou retornando erro em caso de conflito de versão ou intervalo inválido.

## Funcionamento
Valida o `Request`, verifica a versão (se presente) contra `version(before)` e retorna erro de conflito caso não coincida. Divide o texto em linhas para calcular offsets de caracteres, determina (início/fim) conforme a operação (Append/Prepend/Insert/Delete/Replace), valida limites (EOF/linha inválida) e constrói o resultado como `before[..start] + input + before[end..]`.

## Importações
- `super::Operation`: Enum das operações suportadas para determinar o intervalo de edição.
- `super::Request`: Estrutura que contém operação, parâmetros e versão/entrada.
- `super::version`: Função para extrair a versão do conteúdo e comparar com a solicitada.
