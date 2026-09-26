## Resumo
Módulo raiz que declara e reexporta os tipos compartilhados entre as interfaces da aplicação.

## Funcionamento
O arquivo agrupa as sub-rotinas privadas `harness`, `history`, `input`, `invocation` e `workflow`, mantendo suas implementações ocultas dentro do módulo, e em seguida reexporta publicamente apenas os tipos de interesse externo (adaptação de harness e seus erros, blocos e snapshots de histórico, entrada do usuário, invocação e o workflow com seus passos e coordenadas de edição). Isso define a superfície pública de `interfaces` sem duplicar tipos.

## Importações
Nenhuma dependência externa: o módulo expõe somente itens internos via `pub use`.
