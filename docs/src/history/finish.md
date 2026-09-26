## Resumo
Fecha o bloco atualmente em construção, convertendo-o em um `Block` tipado e adicionando-o ao último passo.

## Funcionamento
Toma o par `(marker, lines)` acumulado em `active` (deixando `active` como `None`), removendo uma linha final vazia para evitar quebra de linha residual. O texto é unido com `\n` e classificado pelo marcador: marcadores conhecidos (`ASK`, `INPUT`, `TREE`, `READ`, `EDIT`, `DELETE`) geram as variantes correspondentes; qualquer outro valor (inclusive `None`) resulta em `Block::Output`. O bloco é então anexado ao último passo; se `steps` estiver vazio, retorna `Err("content before a step header")`. Sem conteúdo ativo, retorna `Ok(())` sem efeito colateral.

## Importações
- `super::Block`: Tipos de bloco do parser usados para classificar o conteúdo acumulado.
