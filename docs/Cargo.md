## Resumo
Define o pacote Rust `new-harness`, um CLI neutro de provedor para harnesses de coding-agent.

## Funcionamento
O manifesto declara metadados do pacote (nome, versão, edition 2024), um alvo de biblioteca `new_harness` em `src/lib.rs` e um binário homônimo em `src/main.rs`. As dependências cobrem serialização (JSON/YAML), leitura de `.env`, hash SHA-2 para IDs determinísticos e comparação textual difusa usada em testes de similaridade.

## Importações
- `serde`: Deriva traits de serialização/desserialização das estruturas internas.
- `serde_yaml` / `serde_json`: Leitura e escrita de configuração em YAML e JSON.
- `dotenvy`: Carrega variáveis de ambiente do arquivo `.env` na inicialização.
- `sha2`: Gera hashes estáveis para fingerprint de prompts ou configurações.
- `similar`: Compara strings de forma difusa, útil em testes de deduplicação.
