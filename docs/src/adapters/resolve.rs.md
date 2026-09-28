## Resumo
Resolve o nome de um harness para sua implementação de adaptador.

## Funcionamento
Compara `name` com os identificadores conhecidos e retorna o adaptador correspondente dentro de `Some`. Para nomes não reconhecidos, retorna `None`.

## Importações
- `crate::harness::HarnessAdapter`: Tipo da interface retornada.
- `super`: Adaptadores concretos disponíveis para resolução.
