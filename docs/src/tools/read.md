## Resumo
Expõe a ferramenta `read`, que retorna o conteúdo UTF-8 de um arquivo dentro do diretório de execução.

## Funcionamento
O arquivo funciona apenas como fachada pública do submódulo `read`: declara os módulos internos que compõem a ferramenta (`component`, `component_match`, `components`, `enumerate`, `enumerated_content`, `ignore`, `ignored`, `run`) e reexporta três itens públicos com `pub use`. O ponto de entrada da ferramenta é `read`, oriundo de `run`; `enumerate` e `enumerated_content` também ficam acessíveis ao resto do crate. Como nada é declarado aqui, toda a lógica — validação do caminho, leitura do arquivo e conversão para UTF-8 — permanece encapsulada nos submódulos, deixando este arquivo com o papel de definir a superfície pública da ferramenta e manter os detalhes de implementação privados.

## Importações
- Nenhuma: o arquivo não possui `use`; resolve tudo via `mod` e `pub use` locais.
