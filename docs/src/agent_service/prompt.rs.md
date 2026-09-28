## Resumo
Monta o prompt do agente com instruções de ferramentas e o histórico da conversa.

## Funcionamento
Começa pelas instruções do agente e acrescenta as ferramentas disponíveis conforme suas permissões. Em seguida, inclui os blocos do histórico com seus papéis; pula uma entrada do usuário quando ela vem após uma solicitação de edição ou exclusão.

## Importações
- `crate::history::{Block, History}`: Tipos usados para ler e classificar o histórico.
