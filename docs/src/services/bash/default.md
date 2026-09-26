## Resumo
Implementa `Default` para `BashService` criando uma instância com o shell `"bash"`.

## Funcionamento
A trait `Default` é implementada para `BashService`; o método `default()` delega ao construtor `new("bash")`, fixando o executável do shell. Não há validações, retornos `Result`/`Option` nem efeitos colaterais — é apenas uma inicialização conveniente.

## Importações
- `super::BashService`: Tipo-alvo do `impl`, resolvido a partir do módulo pai.
