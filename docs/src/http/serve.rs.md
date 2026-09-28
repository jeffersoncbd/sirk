## Resumo
Inicia um servidor HTTP que processa requisições em threads separadas.

## Funcionamento
Abre o servidor no endereço informado e retorna o erro como `String` se a inicialização falhar. Para cada requisição, lê o corpo e chama `handle`; se a leitura falhar, prepara uma resposta 400. Envia a resposta como JSON e registra falhas de envio no `stderr`.

## Importações
- `super::handle`: Processa método, caminho e corpo da requisição.
- `super::types`: Fornece a estrutura da resposta HTTP.
- `tiny_http`: Cria o servidor e monta e envia respostas HTTP.
