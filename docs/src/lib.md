## Resumo
Arquivo raiz do crate que apenas declara e expõe publicamente os dez módulos da biblioteca.

## Funcionamento
Não contém lógica executável: serve somente como ponto de entrada da compilação, tornando públicos `adapters`, `agents`, `harness`, `history`, `input`, `interfaces`, `runner`, `services`, `tools` e `workflow` para uso externo via `crate::nome_do_modulo`.

## Importações
- Nenhuma: o arquivo não usa `use`; apenas declara submódulos.
