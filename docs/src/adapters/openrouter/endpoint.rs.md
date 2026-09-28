## Resumo
Monta o endpoint de chat do OpenRouter a partir da configuração disponível.

## Funcionamento
Usa a URL configurada no adaptador, na variável de ambiente ou no arquivo `.env`; se nenhuma existir, usa o valor padrão. Remove barras finais, retorna erro se a URL ficar vazia e acrescenta o caminho necessário conforme o formato da URL.

## Importações
- `DEFAULT_URL`: URL padrão do OpenRouter.
- `OpenRouterAdapter`: Adaptador cujo endpoint é configurado.
- `nonempty`: Ignora valores vazios da variável de ambiente.
- `HarnessAdapter`: Fornece o identificador do adaptador.
- `HarnessError`: Representa erros de configuração.
- `Path`: Indica o diretório para buscar o arquivo `.env`.
