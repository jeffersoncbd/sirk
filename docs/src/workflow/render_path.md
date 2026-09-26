## Resumo
Valida que o step tenha um `path` definido e delega sua renderização com escopo de variáveis.

## Funcionamento
Tenta extrair o campo `path` da struct `Step` via `as_deref`; se ausente (`None`), retorna `Err("tool requires a path")`. Caso contrário, repassa o template para `render_scoped` junto com `outputs` e `locals` opcionais, propagateando o resultado (`Ok` com o texto renderizado ou `Err` vindo da renderização). Sem efeitos colaterais: apenas interpolação de template.

## Importações
- `super::{Step, render_scoped}`: Fornece a struct alvo e a função de renderização com escopo.
- `std::collections::BTreeMap`: Tipa os mapas de `outputs` e `locals` usados na interpolação.
