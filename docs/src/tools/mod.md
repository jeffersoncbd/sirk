## Resumo
Agrava e reexporta os módulos que implementam o despacho de ferramentas da CLI.

## Funcionamento
Apenas declara os submódulos de `tools`, marcando como privados os que são uso interno (`execute`, `execute_with_input`, `request`, `supports`, `format_paths`) e como públicos os ferramentas expostas a outros módulos. Reexporta no nível do módulo as quatro funções de despacho para que chamadores usem `crate::tools::execute` em vez dos caminhos internos.

## Importações
- Nenhuma: o arquivo não usa `use`; expõe submódulos próprios via `pub mod`/`pub use`.
