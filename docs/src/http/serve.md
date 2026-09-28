## Resumo
Inicia um servidor HTTP que processa requisições em threads separadas.

## Funcionamento
Abre o servidor no endereço informado e converte falhas de inicialização em `Err(String)`. Para cada requisição, lê o corpo e chama `handle`; se a leitura falhar, responde com status 400. Envia a resposta como JSON e registra falhas de envio no erro padrão.

## Importações
- `super::handle`: Processa método, caminho e corpo da requisição.
- `super::types`: Fornece a estrutura da resposta HTTP.
- `tiny_http`: Cria o servidor e monta e envia respostas HTTP.
- `std::thread`: Executa cada requisição em uma thread separada.
- `serde_json`: Cria o corpo JSON para erros de leitura.
