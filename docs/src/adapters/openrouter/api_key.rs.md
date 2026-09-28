## Resumo
Obtém a chave da API do OpenRouter ou retorna erro se ela não estiver configurada.

## Funcionamento
Prioriza a chave armazenada no adaptador; se ausente, tenta a variável de ambiente `OPENROUTER_API_KEY` e, por fim, o arquivo dotenv do diretório informado. Retorna a primeira chave encontrada ou `HarnessError::InvalidConfiguration` se nenhuma estiver disponível.

## Importações
- `OpenRouterAdapter`: fornece a configuração e o acesso ao arquivo dotenv.
- `nonempty`: descarta valores vazios da variável de ambiente.
- `HarnessAdapter`: fornece o identificador do adaptador.
- `HarnessError`: representa falhas de configuração.
- `Path`: representa o diretório usado para buscar o arquivo dotenv.
