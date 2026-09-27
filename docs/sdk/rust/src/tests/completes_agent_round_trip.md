## Resumo
Verifica a comunicação de ida e volta entre `Sirk` e um servidor simulado.

## Funcionamento
Cria um diretório temporário e um servidor executável que responde com `"documented"`. Inicia `Sirk`, envia uma solicitação de agente e confirma a resposta; ao final, encerra a instância e remove o diretório. Falhas nas operações causam panic.

## Importações
- `crate::Sirk`: Inicia o cliente e solicita a execução do agente.
- `std::fs`: Cria, configura e remove arquivos e diretórios temporários.
- `PermissionsExt`: Define permissões de execução para o servidor simulado.
- `SystemTime` e `UNIX_EPOCH`: Geram um nome temporário único.
