### Resumo

O `Cargo.toml` é o manifesto da crate Rust `new-harness`. Ele define metadados do pacote, a versão da edição Rust, a biblioteca principal, o executável e as dependências externas.

### Funcionamento

O projeto utiliza a edição 2024 do Rust e possui dois alvos de compilação:

- Uma biblioteca chamada `new_harness`, cujo código começa em `src/lib.rs`.
- Um executável chamado `new-harness`, definido em `src/main.rs`.

As dependências fornecem suporte a:

- Serialização e desserialização com `serde`, `serde_yaml` e `serde_json`.
- Leitura de variáveis de ambiente com `dotenvy`.
- Cálculo de hashes com `sha2`.
- Comparação de textos ou estruturas textuais com `similar`.

O arquivo não contém lógica de execução, tratamento de erros ou regras de negócio; apenas configura como o Cargo deve compilar e organizar o projeto.

### Componentes principais

- `[package]`: define o nome, versão, edição do Rust e descrição do pacote.
- `[lib]`: declara a crate de biblioteca `new_harness` e seu arquivo de entrada `src/lib.rs`.
- `[dependencies]`: lista as crates externas e suas versões/recursos habilitados.
- `[[bin]]`: declara o binário `new-harness` e seu arquivo de entrada `src/main.rs`.

### integrações

A biblioteca pública do pacote é exposta como a crate `new_harness`. O binário `new-harness` constitui o ponto de entrada executável separado.

As integrações externas são feitas pelas crates `serde`, `serde_yaml`, `serde_json`, `dotenvy`, `sha2` e `similar`. O manifesto não expõe structs, enums, traits ou funções diretamente; esses símbolos devem estar definidos nos arquivos Rust indicados.
