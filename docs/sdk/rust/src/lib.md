## Resumo
Define o módulo raiz da biblioteca e expõe `Error` e `Sirk` publicamente.

## Funcionamento
Declara os módulos internos da biblioteca e reexporta `Error` e `Sirk`; os testes ficam condicionados à compilação de testes.

## Importações
- `agent`: Declara o módulo interno de agentes.
- `drop`: Declara o módulo interno `drop`.
- `error`: Fornece o tipo `Error`, reexportado publicamente.
- `protocol`: Declara o módulo interno de protocolo.
- `sirk`: Fornece o tipo `Sirk`, reexportado publicamente.
- `start`: Declara o módulo interno de inicialização.
