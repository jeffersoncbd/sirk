## Resumo
Valida um `Workflow` verificando versão e presença de passos antes de delegar a validação estrutural.

## Funcionamento
A função recusa workflows cuja `version` não seja `1` e workflows sem nenhum passo, retornando `String` descritiva em ambos os casos. Se passar nas duas checagens, chama `validate_steps` supplying um mapa de índices (`BTreeMap`), um contexto inicial `None` e um conjunto de visitados (`BTreeSet`) já vazios — todos mutáveis para accumulating estado durante a recursão — e propaga o resultado (`Ok(())` ou `Err`) sem alteração. Não há efeitos colaterais: apenas lê `self` e constrói estruturas auxiliares em memória.

## Importações
- `super::{Workflow, validate_steps}`: Tipo alvo do método e rotina de validação dos passos.
- `std::collections::BTreeMap`: Mapa ordenado mutável para indexes/estados dos passos.
- `std::collections::BTreeSet`: Conjunto ordenado mutável de itens já visitados.
