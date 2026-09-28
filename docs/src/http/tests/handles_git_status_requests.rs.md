## Resumo
O arquivo testa se a rota de status do Git retorna os arquivos visíveis do diretório.

## Funcionamento
O teste cria um repositório temporário, adiciona arquivos visíveis e um arquivo ignorado por `.treeignore`, e envia uma requisição `POST` para `/v1/git/status`. Verifica que a resposta tem status 200 e contém apenas `.treeignore` e `visible.rs`, então remove o diretório temporário.

## Importações
- `handle`: executa a requisição HTTP simulada.
- `BashService`, `Invocation`: inicializam o repositório Git.
- `fs`, `io`: criam arquivos e descartam a saída do comando.
- `SystemTime`, `UNIX_EPOCH`: tornam único o nome do diretório temporário.
- `#[test]`: marca a função como teste.
