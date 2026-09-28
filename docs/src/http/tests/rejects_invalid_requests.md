## Resumo
O arquivo não contém função principal; seu teste verifica respostas HTTP para quatro requisições.

## Funcionamento
Confere que `/health` retorna 200, rota inexistente 404, método inválido 405 e corpo inválido 400.

## Importações
- `handle`: Executa as requisições HTTP testadas.
