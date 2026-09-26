## Resumo
Valida estruturalmente os blocos de um transcript persistido, garantindo que todo passo do workflow tenha histórico correspondente.

## Funcionamento
`validate_blocks` percorre os passos do workflow via `visit` sobre o snapshot do histórico, consumindo um cursor incremental. Se ao final o cursor não coincidir com o total de passos registrados, significa que existem entradas de histórico "pendentes" cuja ordem ficou inconsistente (passo editado com histórico posterior) — nesse caso retorna `Err` com orientação para remover o resultado editado e tudo em diante. Sucesso retorna `Ok(())`. A função é `pub(super)`, restrita ao módulo pai `runner`, e delega a análise detalhada de cada tipo de bloco aos submódulos auxiliares.

## Importações
- `crate::history::History`: Fonte dos dados de transcript e snapshot validados.
- `visit::visit`: Percorre blocos incrementando o cursor de validação.
