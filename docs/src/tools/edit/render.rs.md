## Resumo
Renderiza um diff, destacando alterações e escapando caracteres de controle.

## Funcionamento
Após a primeira linha de cabeçalho `@@`, colore linhas adicionadas e removidas quando `color` está ativo. Escapa caracteres de controle vindos do conteúdo, preserva quebras de linha e retorna o texto renderizado.

## Importações
- Nenhuma: usa apenas tipos da biblioteca padrão.
