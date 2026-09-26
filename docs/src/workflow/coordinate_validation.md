## Resumo
Valida se uma coordenada de edição é um template interpolado e devolve o número de linhas afetadas.

## Funcionamento
Se a coordenada for do tipo `Template` e contiver `"{{"`, delega a renderização a `render_scoped` (propagando o erro com `?`) e retorna `usize::MAX` quando o nome é `"end"`, indicando que o blocoWaits for anything else. Should I go ahead with the Portuguese documentation?## Resumo
Renderiza uma coordenada de edição durante a validação, retornando a contagem de linhas afetadas.

## Funcionamento
Quando a coordenada é do tipo `Template` e contém `"{{"`, o conteúdo é renderizado por `render_scoped` e seu erro propagado via `?`; nesse caso retorna `usize::MAX` se o nome for `"end"` (bloco de encerramento) e `1` caso contrário. Para qualquer outro caso (template estático ou coordenada não-template), o resultado de `self.render` é devolvido diretamente, mantendo o `Result<usize, String>` original.

## Importações
- `super::EditCoordinate`: Tipo da qual este método é uma extensão de renderização.
- `super::render_scoped`: Renderiza templates com `outputs`/`locals` e sinaliza erros.
- `std::collections::BTreeMap`: Tipa os mapas ordenados de saídas e variáveis locais.
