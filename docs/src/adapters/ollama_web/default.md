## Resumo
Fornece os valores padrão do `OllamaWebAdapter`: executável `curl` e ausência de URL base e chave de API.

## Funcionamento
A implementação de `Default` monta uma nova instância com o campo `executable` preenchido com `"curl"` (via `to_owned`) e os campos opcionais `base_url` e `api_key` inicializados como `None`. Não há validações, I/O, erros (`Result`/`Option` retornados) ou efeitos colaterais; o comportamento pode ser sobrescrito pelo construtor explícito ou ajuste posterior dos campos.

## Importações
- `super::OllamaWebAdapter`: Tipo alvo da implementação do trait `Default`.
