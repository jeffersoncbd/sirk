## Resumo
Gera uma versão de `value` protegida para uso em shell.

## Funcionamento
Se `value` começar com `$` seguido de um nome válido presente em `environment`, mantém a variável entre aspas duplas. Caso contrário, envolve o valor em aspas simples e escapa apóstrofos. Sempre retorna uma `String`.

## Importações
- `std::collections::BTreeMap`: Consulta variáveis disponíveis no ambiente.
