## Resumo
Verifica se um `&str` é um identificador válido (não vazio, apenas alfanumérico ASCII, `_` ou `-`).

## Funcionamento
Retorna `true` apenas se a string não for vazia e todos os seus caracteres forem alfanuméricos ASCII ou os símbolos `_`/`-`. Não há alocação de memória nem efeitos colaterais; qualquer caractere Unicode fora do ASCII (inclusive acentos) invalida o ID. Como devolve `bool`, não há `Result`/`Option` a tratar.

## Importações
- Nenhuma: a função usa apenas métodos padrão de `str` (`is_empty`, `chars`) e `char` (`is_ascii_alphanumeric`).
