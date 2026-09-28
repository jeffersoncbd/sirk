## Resumo
O arquivo contém um teste que verifica a geração e a renderização de diferenças de texto.

## Funcionamento
O teste confirma que a diferença mostra as linhas removida e adicionada, que a renderização sem cores preserva o resultado e que a colorida destaca essas linhas. Também verifica a remoção de sequências de escape e a indicação de ausência de quebra de linha final.

## Importações
- `super::*`: Importa os itens do módulo pai usados pelo teste.
- `crate::tools::edit::render::render`: Renderiza diferenças com ou sem cores.
