## Resumo
Renderiza um diff de texto, opcionalmente colorindo linhas adicionadas (verde) e removidas (vermelho) dentro de hunks.

## Funcionamento
Percorre o diff linha a linha preservando quebras finais; ao encontrar um cabeçalho `@@` habilita a coloração. Dentro do hunk, linhas com `+` recebem o código ANSI 42 e com `-` o 41, involving a cor ao redor do conteúdo. Cada caractere é escapado via `escape_default` quando é de controle (exceto `\n` e `\t`), evitando que sequências vindas do arquivo-fonte sejam interpretadas como ANSI. Retorna a string completa; `color = false` devolve o diff sem escapes.

## Importações
- `String`: buffer de saída acumulado da renderização.
