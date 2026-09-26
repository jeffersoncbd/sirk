## Resumo
Converte um nome de harness em uma instância `Box<dyn HarnessAdapter>` correspondente.

## Funcionamento
A função faz um `match` sobre o identificador recebido e devolve, para cada nome reconhecido (`codex`, `ollama`, `ollama-web`, `opencode`, `openrouter`), uma nova caixa contendo o adapter correspondente construído via `default()`. Nomes desconhecidos ou ausentes resultam em `None`, sem panicking nem efeitos colaterais; a trait permite ao consumidor tratar todos os adapters uniformemente via `id()`.

## Importações
- `HarnessAdapter`: Trait que define o contrato comum e permite o downcast para `Box<dyn>`.
- `CodexAdapter`, `OllamaAdapter`, `OllamaWebAdapter`, `OpenCodeAdapter`, `OpenRouterAdapter`: Implementações concretas instanciadas conforme o nome recebido.
