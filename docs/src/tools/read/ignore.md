## Resumo
Verifica se um `path` corresponde a um padrão de ignore (glob) de arquivo ou diretório.

## Funcionamento
Normaliza o padrão removendo `/` inicial e opcionalmente a barra final para obter o diretório alvo. O caminho e o padrão são divididos em segmentos por `/`. Se o padrão tiver um único segmento, compara esse segmento contra cada parte do caminho (correspondência em qualquer nível); caso contrário, delega a comparação segmento a segmento,emblies indicando se o alvo é um diretório (`true`) quando o padrão termina com `/` — o que faz os segmentos filhos serem aceitos. Retorna `bool`.

## Importações
- `std`: não há imports; usa apenas `str` e `Vec` da biblioteca padrão.
