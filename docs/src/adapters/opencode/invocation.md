## Resumo
Monta a invocação do OpenCode para executar uma solicitação do harness.

## Funcionamento
Inicia com `run` e define um título fixo para a sessão. Se solicitado, ativa saída JSON e inclui o modelo; depois acrescenta `--` e o prompt. Retorna a invocação com o executável e diretório de trabalho configurados, sem variáveis de ambiente adicionais.

## Importações
- `super::OpenCodeAdapter`: Executável configurado do adaptador.
- `crate::harness`: Tipos da interface e da invocação do harness.
