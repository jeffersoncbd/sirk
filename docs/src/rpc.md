## Resumo
Expõe `serve` como ponto de entrada do transporte JSON-RPC.

## Funcionamento
Declara os módulos internos do transporte e reexporta `serve` para uso externo; não define uma função neste arquivo.

## Importações
- `execute`: Módulo interno de execução.
- `handle`: Módulo interno de tratamento de mensagens.
- `run_agent`: Módulo interno de execução de agentes.
- `serve`: Função reexportada como ponto de entrada.
- `types`: Tipos internos do transporte.
