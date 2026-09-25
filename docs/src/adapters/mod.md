### Resumo

`src/adapters/mod.rs` funciona como o módulo de registro e resolução dos adaptadores de harness disponíveis na aplicação. Ele declara os módulos internos, reexporta seus adaptadores públicos e permite obter uma implementação de `HarnessAdapter` a partir de um nome textual.

### Funcionamento

O arquivo declara cinco módulos privados: `codex`, `ollama`, `ollama_web`, `opencode` e `openrouter`.

A função `resolve` recebe o nome de um adaptador e usa `match` para criar sua implementação correspondente com `Default`. O resultado é encapsulado em `Box<dyn HarnessAdapter>`, permitindo trabalhar de forma genérica com diferentes implementações da mesma trait.

Quando o nome não é reconhecido, a função retorna `None`.

A constante `AVAILABLE` mantém a lista dos nomes aceitos. Os testes verificam a resolução de adaptadores web e confirmam que esses nomes também estão anunciados na lista disponível.

### Componentes principais

- `mod codex`, `mod ollama`, `mod ollama_web`, `mod opencode`, `mod openrouter`: declara os módulos internos que contêm as implementações concretas dos adaptadores.
- `pub use ...`: reexporta publicamente `CodexAdapter`, `OllamaAdapter`, `OllamaWebAdapter`, `OpenCodeAdapter` e `OpenRouterAdapter`.
- `resolve(name: &str) -> Option<Box<dyn HarnessAdapter>>`: converte um identificador textual em um adaptador concreto armazenado como trait object.
- `AVAILABLE`: fatia pública com os identificadores reconhecidos por `resolve`.
- Módulo de testes: valida a criação de adaptadores e a presença dos nomes correspondentes em `AVAILABLE`.

### integrações

O arquivo integra-se com a trait interna `crate::harness::HarnessAdapter` e com as implementações específicas dos módulos de adaptadores.

As partes públicas expostas são:

- Os cinco tipos de adaptador reexportados.
- A função `resolve`.
- A constante `AVAILABLE`.

O comportamento detalhado de cada adaptador depende dos arquivos dos módulos correspondentes, que não estão incluídos no conteúdo analisado.
