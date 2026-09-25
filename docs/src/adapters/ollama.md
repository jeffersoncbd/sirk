### Resumo

O arquivo implementa o adaptador do Ollama para o harness. Sua responsabilidade é converter um `RunRequest` genérico em uma `Invocation` específica para executar o comando `ollama run`.

### Funcionamento

O adaptador exige que a requisição informe um modelo. Caso contrário, retorna `HarnessError::MissingModel`.

Ele também rejeita requisições com `event_stream` habilitado, pois esse recurso não é suportado pelo adaptador. Nessa situação, retorna `HarnessError::UnsupportedOption`.

Quando a requisição é válida, monta uma invocação com:

- programa configurado, normalmente `ollama`;
- argumentos `run`, o nome do modelo e o prompt;
- diretório de trabalho recebido na requisição;
- ambiente de execução vazio.

O arquivo não executa diretamente processos; apenas constrói a descrição da execução. Os testes verificam a tradução correta, a exigência de modelo e a rejeição de streams de eventos.

### Componentes principais

- `OllamaAdapter`: struct pública que armazena o nome ou caminho do executável do Ollama.
- `Default for OllamaAdapter`: cria o adaptador usando o executável padrão `"ollama"`.
- `OllamaAdapter::new`: construtor público que permite definir outro executável.
- `HarnessAdapter for OllamaAdapter`:
  - `id`: retorna o identificador `"ollama"`;
  - `invocation`: valida o `RunRequest` e produz uma `Invocation` ou um `HarnessError`.
- Módulo privado `tests`: contém testes unitários para conversão de requisições e tratamento de erros.

### integrações

O arquivo utiliza os tipos internos `HarnessAdapter`, `HarnessError`, `Invocation` e `RunRequest`, definidos no módulo `crate::harness`.

A struct `OllamaAdapter` e seu construtor são públicos. A implementação pública de `HarnessAdapter` expõe o identificador do adaptador e a capacidade de transformar requisições em invocações do Ollama.
