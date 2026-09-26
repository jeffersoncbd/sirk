## Resumo
Gera uma representação numerada de um conteúdo textual, prefixando cada linha com seu índice (base 1) e o cabeçalho `Line | Content`.

## Funcionamento
A função inicia a saída com o cabeçalho `"Line | Content\n"` e percorre o resultado de `split_inclusive('\n')`, que preserva o terminador de linha original de cada trecho (inclusive a ausência dele na última linha, quando o conteúdo não termina com `\n`). Para cada trecho, escreve `índice + 1`, o separador `" | "` e o conteúdo exato. Não há validações, erros ou efeitos colaterais: a operação é pura e sempre bem-sucedida, retornando uma `String` — a ausência de `Result`/`Option` indica que entradas vazias ou sem newline final são casos válidos.

## Importações
Nenhuma importação explícita: a função depende apenas do crate `std` (tipos `String` e `str`), usado de forma implícita e sem necessidade de `use`.
