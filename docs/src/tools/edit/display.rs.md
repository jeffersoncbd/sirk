## Resumo
Exibe uma diferença renderizada, usando cores quando o terminal permite.

## Funcionamento
Verifica se a saída padrão é um terminal e se `NO_COLOR` está ausente; usa essa condição para renderizar a diferença e imprime o resultado na saída padrão.

## Importações
- `std::io::IsTerminal`: Verifica se a saída padrão é um terminal.
- `super::render::render`: Renderiza a diferença com ou sem cores.
