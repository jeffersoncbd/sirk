### Resumo

Este arquivo `Cargo.toml` define a configuração do pacote Rust `new-harness`, uma CLI neutra em relação a provedores para harnesses de agentes de programação. Ele configura simultaneamente uma biblioteca e um executável.

### Funcionamento

O projeto usa a edição 2024 do Rust e possui versão `0.1.0`.

A configuração define:

- Uma biblioteca chamada `new_harness`, cujo código está em `src/lib.rs`.
- Um executável chamado `new-harness`, cujo ponto de entrada está em `src/main.rs`.
- Dependências para serialização, processamento de JSON/YAML, hashing e comparação de textos.

O arquivo apenas descreve metadados e componentes de compilação; não contém lógica de execução.

### Componentes principais

- `[package]`: define nome, versão, edição e descrição do pacote.
- `[lib]`: configura a crate de biblioteca `new_harness` e aponta para `src/lib.rs`.
- `[[bin]]`: configura o binário `new-harness` e aponta para `src/main.rs`.
- `serde`: serialização e desserialização com suporte a `derive`.
- `serde_yaml`: leitura e escrita de dados no formato YAML.
- `serde_json`: processamento de JSON.
- `sha2`: geração de hashes SHA-2.
- `similar`: comparação e geração de diferenças entre textos ou conteúdos.

### Dependências e integrações

O pacote está estruturado para oferecer uma biblioteca reutilizável e uma interface de linha de comando. Os detalhes de integração entre `src/lib.rs`, `src/main.rs` e as dependências não aparecem neste arquivo.

### Observações

Não é possível determinar, apenas pelo `Cargo.toml`, como os dados são processados, quais hashes são usados, como a CLI funciona ou quais regras de negócio existem.
