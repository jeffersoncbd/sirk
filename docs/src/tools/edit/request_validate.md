## Resumo
Valida se um pedido de edição respeita os campos obrigatórios e as coordenadas compatíveis com sua operação.

## Funcionamento
`validate` percorre quatro checagens: o caminho não pode ser vazio após `trim`; as coordenadas devem bater com a operação — `Insert` exige `line > 0` sem `start`/`end`, `Delete`/`Replace` exigem `start`/`end` válidos com `end >= start` e sem `line`, e `Prepend`/`Append` não aceitam coordenada alguma; `Delete` recusa `input` não vazio; e, exceto em `Append`/`Prepend`, é obrigatório um `version` que seja um digest hexadecimal de 64 caracteres (SHA-256), evitando edições sem knowledge da versão lida. Erros retornam `Err(String)` com mensagem descritiva; caso contrário `Ok(())`. Nenhum efeito colateral.

## Importações
- `super::{Operation, Request}`: Tipos do módulo pai; `Operation` seleciona as regras e `Request` fornece os campos validados.
