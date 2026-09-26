## Resumo
Módulo agregador que declara e reexporta os adaptadores de provedores de LLM, além da lista `AVAILABLE` de provedores suportados.

## Funcionamento
Declara os submódulos (`codex`, `ollama`, `ollama_web`, `opencode`, `openrouter`, `resolve`) e reexporta publicamente cada adaptador e a função `resolve`, restricting a API externa ao módulo pai. A constante `AVAILABLE` mantém a lista canônica de nomes de provedores usados em validações e na interface.

## Importações
- `codex`: Adaptador para o provedor Codex.
- `ollama`: Adaptador para o Ollama local.
- `ollama_web`: Adaptador para o Ollama via web.
- `opencode`: Adaptador para o OpenCode.
- `openrouter`: Adaptador para o OpenRouter.
- `resolve`: Resolução do adaptador a partir do nome.
