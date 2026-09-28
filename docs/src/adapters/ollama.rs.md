## Resumo
Converte uma solicitação em uma invocação da CLI do Ollama.

## Funcionamento
Exige um modelo e rejeita solicitações com fluxo de eventos; caso contrário, monta a execução de `ollama run` com o modelo, o prompt e o diretório de trabalho, sem variáveis de ambiente adicionais.

## Importações
- `crate::harness`: Tipos da solicitação, invocação e erros do adaptador.
- `default`: Implementação padrão do adaptador.
- `new`: Construtor do adaptador.
