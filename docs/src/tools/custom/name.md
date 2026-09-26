## Resumo
Valida se um nome é não vazio e contém apenas caracteres alfanuméricos ASCII, `_` ou `-`.

## Funcionamento
Retorna `false` imediatamente para strings vazias. Caso contrário, percorre os caracteres com `chars()` e exige que todos satisfaçam `is_ascii_alphanumeric()` ou estejam entre `'_'` e `'-'`. Basta um caractere fora dessas regras para retornar `false`; qualquer resultado é derivado apenas da entrada, sem efeitos colaterais.

## Importações
- Nenhuma: a função usa apenas métodos de `str` da biblioteca padrão.
