### Resumo

Este arquivo implementa o adaptador do Ollama para o harness. Sua responsabilidade é converter uma requisição genérica (`RunRequest`) em uma invocação específica da CLI `ollama`.

### Funcionamento

O `OllamaAdapter` armazena o nome do executável, usando `"ollama"` por padrão. Ao receber uma requisição:

1. Verifica se um modelo foi informado.
2. Rejeita requisições que solicitam *event streams*, pois esse recurso não é suportado pelo adaptador.
3. Cria uma `Invocation` equivalente a:

```text
ollama run <modelo> <prompt>
```

O diretório de trabalho da requisição é preservado na invocação.

### Componentes principais

- `OllamaAdapter`: struct pública que representa o adaptador do Ollama.
- `Default for OllamaAdapter`: cria o adaptador usando o executável padrão `"ollama"`.
- `OllamaAdapter::new`: permite configurar outro caminho ou nome de executável.
- `HarnessAdapter for OllamaAdapter`:
  - `id`: retorna o identificador `"ollama"`.
  - `invocation`: transforma `RunRequest` em `Invocation` ou retorna `HarnessError`.
- Módulo de testes:
  - Verifica a geração correta do comando `ollama run`.
  - Verifica o erro quando o modelo está ausente.
  - Verifica a rejeição de *event streams*.

### Dependências e integrações

- `crate::harness` fornece:
  - `HarnessAdapter`, trait que o adaptador implementa.
  - `HarnessError`, para erros de validação.
  - `Invocation`, representação do processo a ser executado.
  - `RunRequest`, requisição independente do provedor.
- `std::path::PathBuf` é usado apenas nos testes para configurar o diretório de trabalho.
- A execução efetiva do processo não ocorre neste arquivo; ele apenas constrói a descrição da invocação.

### Observações

O tratamento de erros usa `Result` e `ok_or`, retornando erros estruturados para modelo ausente e opção não suportada. Os argumentos são armazenados separadamente em um `Vec<String>`, sem interpolação em shell.
