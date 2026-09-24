### Resumo

Este arquivo define a interface comum para executar diferentes CLIs de agentes de código. Ele padroniza os dados de uma execução, o contrato dos adaptadores e os erros relacionados a opções não suportadas.

### Funcionamento

`RunRequest` reúne as informações necessárias para iniciar uma execução: prompt, diretório de trabalho, modelo opcional e indicação de stream de eventos.

Cada implementação de `HarnessAdapter` representa uma integração com um harness específico. Ela deve:

- retornar seu identificador por `id()`;
- transformar um `RunRequest` em uma `Invocation` por meio de `invocation()`.

A criação da `Invocation` pode falhar com `HarnessError`, atualmente usado para indicar que determinado adaptador não oferece suporte a uma opção solicitada.

### Componentes principais

- `RunRequest`: struct pública com os parâmetros provider-neutral da execução.
- `HarnessAdapter`: trait pública que define a fronteira de integração com CLIs de agentes.
- `HarnessError`: enum público para erros de configuração ou compatibilidade do adaptador.
- `Invocation`: tipo reexportado publicamente de `crate::services`, usado para representar a execução de um processo externo.
- Implementação de `Display`: produz mensagens como `adapter \`...\` does not support \`...\``.
- Implementação de `std::error::Error`: permite usar `HarnessError` como erro idiomático de Rust.

### Dependências e integrações

- `std::fmt`: usado para implementar `Display`.
- `std::path::PathBuf`: representa o diretório de trabalho.
- `crate::services::Invocation`: conecta este contrato ao serviço interno responsável por executar processos.
- Os adaptadores concretos devem implementar `HarnessAdapter`, provavelmente em módulos específicos de `crate::adapters`.

### Observações

O arquivo não executa processos nem realiza comunicação externa diretamente. Ele apenas define contratos e tipos compartilhados. O comportamento detalhado de cada harness depende das implementações concretas de `HarnessAdapter`, que não estão presentes no conteúdo analisado.
