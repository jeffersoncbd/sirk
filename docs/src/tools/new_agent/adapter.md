## Resumo
Pede ao usuário que escolha um adaptador válido (de `adapters::AVAILABLE`) até a resposta, em minúsculas, corresponder a um item da lista.

## Funcionamento
Monta a pergunta inicial já listando os adaptadores disponíveis e entra em laço: lê a entrada via `input.ask` (propagando o `Result` com `?`), aplica `trim` e `to_lowercase` e compara com `AVAILABLE.contains`. Em caso de acerto retorna `Ok(value)`; caso contrário, reformula a mensagem de erro (`Unsupported adapter...`) e repete a pergunta, indefinidamente.

## Importações
- `adapters`: Fornece a constante `AVAILABLE` com os adaptadores suportados.
- `crate::input::UserInput`: Trait que abstrai a leitura da resposta do usuário via `ask`.
