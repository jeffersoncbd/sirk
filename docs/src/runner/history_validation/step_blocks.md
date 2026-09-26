## Resumo
Valida a sequência de blocos de uma etapa da conversa e indica se ela está completa.

## Funcionamento
Percorre os blocos em ordem aplicando regras por tipo: `Ask` só é aceito no início e fora de etapa de ferramenta; `Input` exige que o estado espere entrada, ainda não esteja completo e que não haja pedido de ferramenta pendente; `Output` exige estado inverso e reinicia a espera, marcando conclusão quando não é pergunta nem requisição; `Tree`/`Read`/`Edit` só aparecem após a saída da ferramenta correspondente; `Delete` é sempre rejeitado. A primeira violação retorna `Err` com orientação para remover a resposta editada e o que vem depois; caso contrário devolve `Ok(complete)`.

## Importações
- `question`: Indica se o texto de saída é uma pergunta ao usuário.
- `Block`: Variante do bloco de conversa analisada na validação.
- `crate::tools::request`: Detecta pedidos de ferramenta e seu tipo no texto.
