### Resumo

O arquivo implementa o adaptador que transforma requisições genéricas do harness em comandos executáveis pela CLI do OpenCode.

### Funcionamento

`OpenCodeAdapter` armazena o nome do executável e, por padrão, usa `opencode`. Ao receber um `RunRequest`, constrói uma `Invocation` para executar:

- o subcomando `run`;
- um título fixo (`new-harness`);
- saída em JSON quando `event_stream` está habilitado;
- o modelo informado, se houver;
- o prompt após `--`, evitando sua interpretação como opção da CLI.

O adaptador não adiciona a opção `--auto`, pois ela habilitaria aprovações automáticas de permissões. A função retorna `Result<Invocation, HarnessError>`, embora a implementação atual sempre produza `Ok`.

Os testes verificam a tradução das opções e garantem que a aprovação automática não seja ativada.

### Componentes principais

- `OpenCodeAdapter`: struct pública que contém o caminho ou nome do executável do OpenCode.
- `Default::default`: cria um adaptador configurado com o executável `opencode`.
- `OpenCodeAdapter::new`: construtor público que aceita qualquer tipo conversível para `String`.
- `HarnessAdapter::id`: identifica o adaptador com o valor `"opencode"`.
- `HarnessAdapter::invocation`: converte um `RunRequest` em uma `Invocation`, preservando prompt, diretório de trabalho e modelo.
- Módulo `tests`: contém testes unitários privados para a montagem dos argumentos.

### integrações

O arquivo importa do módulo interno `crate::harness` os tipos `HarnessAdapter`, `HarnessError`, `Invocation` e `RunRequest`.

A implementação expõe publicamente `OpenCodeAdapter`, seu construtor `new` e a implementação do trait `HarnessAdapter`. A execução efetiva do processo e o significado completo de `Invocation` dependem do restante do módulo `harness`, que não está presente no conteúdo analisado.
