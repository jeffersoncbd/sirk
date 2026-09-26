## Resumo
Retorna o nome da etapa (`&str`), resolvido a partir dos campos `agent`, `tool` ou `custom_tool` do struct `Step`.

## Funcionamento
A função aplica encadeamento de `Option::as_deref` para extrair a `&str` do primeiro campo preenchido, seguindo a prioridade `agent` → `tool` → `custom_tool`. Se todos forem `None`, devolve o literal `"invalid"` como valor padrão, garantindo retorno válido em vez de `Option`. Não há erros (`Result`) nem efeitos colaterais; é apenas uma leitura dos campos. O receiver `&self` evita clonagem, mas o `&str`植 returned tem prazo de vida atrelado a `self`.

## Importações
- `super::Step`: Struct dono dos campos `agent`, `tool` e `custom_tool`咨询yi consultados.
