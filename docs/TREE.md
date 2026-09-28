.env.example - Documenta as variáveis de ambiente para configurar URLs e chaves de API dos serviços Ollama e OpenRouter.
Cargo.toml - Define o pacote `sirk`, seus pontos de entrada e as dependências usadas pela biblioteca e pelo executável.
src/adapters/codex.rs - Configura a execução de solicitações pelo comando `codex exec`, montando seus argumentos e prompt.
src/adapters/codex/default.rs - Define a configuração padrão do CodexAdapter para usar o comando `codex`.
src/adapters/codex/new.rs - Cria uma instância de `CodexAdapter` armazenando o executável informado como texto.
src/adapters/mod.rs - Centraliza os adaptadores disponíveis, reexporta sua resolução e lista os nomes aceitos.
src/adapters/ollama.rs - Converte solicitações compatíveis em invocações de `ollama run` com modelo, prompt e diretório de trabalho.
src/adapters/ollama/default.rs - Cria e retorna uma instância padrão de `OllamaAdapter` identificada pelo nome “ollama”.
src/adapters/ollama/new.rs - Inicializa um OllamaAdapter armazenando o executável informado como texto.
src/adapters/ollama_web.rs - Adapta solicitações para chamadas não streaming à API web do Ollama e extrai a resposta recebida.
src/adapters/ollama_web/api_key.rs - Obtém a chave da API do Ollama, priorizando a configuração do adaptador, o ambiente e o arquivo dotenv.
src/adapters/ollama_web/default.rs - Define os valores padrão do adaptador Ollama Web, usando `curl` e sem URL base ou chave de API.
src/adapters/ollama_web/dotenv.rs - Obtém do arquivo `.env` de um diretório o valor não vazio de uma chave, retornando ausência quando não encontrada.
src/adapters/ollama_web/endpoint.rs - Monta a URL do endpoint de geração do Ollama Web com base nas configurações disponíveis.
src/adapters/ollama_web/new.rs - Cria um `OllamaWebAdapter` com os valores informados e normaliza a URL e a chave de API opcionais.
src/adapters/ollama_web/nonempty.rs - Retorna o texto original em `Some` quando contém algum caractere não branco; caso contrário, retorna `None`.
src/adapters/opencode.rs - Declara o adaptador OpenCode, responsável por representar solicitações como chamadas à CLI OpenCode.
src/adapters/opencode/default.rs - Cria o adaptador OpenCode padrão configurado para usar o comando `opencode`.
src/adapters/opencode/id.rs - Define, por meio de macro, o método que identifica o adaptador pelo nome estático “opencode”.
src/adapters/opencode/invocation.rs - Monta a invocação do OpenCode para executar uma solicitação do harness com suas opções, prompt e diretório de trabalho.
src/adapters/opencode/new.rs - Cria e retorna um OpenCodeAdapter com o executável informado armazenado como texto.
src/adapters/opencode/tests.rs - Declara e organiza os módulos de teste do adaptador OpenCode com símbolos e tipos de suporte.
src/adapters/opencode/tests/does_not_enable_automatic_permission_approval.rs - Verifica que o adaptador OpenCode não ativa a aprovação automática de permissões.
src/adapters/opencode/tests/translates_generic_options_to_opencode_run.rs - Verifica se opções genéricas são convertidas corretamente em uma invocação esperada do OpenCode.
src/adapters/openrouter.rs - Converte solicitações do harness em chamadas não-streaming à API da OpenRouter e extrai o conteúdo da resposta.
src/adapters/openrouter/api_key.rs - Obtém a chave da API do OpenRouter pela configuração disponível ou retorna erro se nenhuma for encontrada.
src/adapters/openrouter/default.rs - Define o adaptador OpenRouter com `curl` e sem URL base ou chave de API configuradas.
src/adapters/openrouter/dotenv.rs - Lê do arquivo `.env` de um diretório o valor não vazio de uma chave, se ela estiver definida.
src/adapters/openrouter/endpoint.rs - Configura o endpoint de chat do OpenRouter com a URL disponível, aplicando o caminho e o padrão necessários.
src/adapters/openrouter/new.rs - Cria um adaptador OpenRouter com executável, URL base e chave de API opcionais, descartando valores vazios.
src/adapters/openrouter/nonempty.rs - Retorna a string original se houver conteúdo além de espaços; caso contrário, retorna `None`.
src/adapters/resolve.rs - Localiza o adaptador de harness pelo nome e retorna `None` quando não houver correspondência.
src/agent_service.rs - Disponibiliza internamente `run::run` como ponto de entrada para a execução de agentes.
src/agent_service/conversation.rs - Conduz a conversa com o agente, processando respostas e ferramentas até obter uma resposta final.
src/agent_service/delete_request.rs - Processa pedidos de exclusão de arquivo, verificando a autorização para execução forçada e evitando repetições.
src/agent_service/edit_request.rs - Prepara e aplica uma edição solicitada, registrando-a no histórico e evitando repetir alterações já concluídas.
src/agent_service/execute.rs - Executa uma invocação e retorna a saída padrão apenas em caso de sucesso, convertendo falhas em mensagens.
src/agent_service/prompt.rs - Monta o prompt do agente com instruções, ferramentas permitidas e histórico filtrado da conversa.
src/agent_service/run.rs - Executa uma conversa com o agente carregado para o diretório indicado, após validar seu adaptador e criar o histórico.
src/agents.rs - Valida os metadados e as instruções de uma definição textual e a converte em um agente.
src/agents/call_prefix.rs - Desserializa `call_prefix` como lista de argumentos, aceitando string ou lista e tratando ausência como lista vazia.
src/agents/delete_permission.rs - Desserializa `"allow"` como `true` e rejeita qualquer outro valor com um erro descritivo.
src/agents/edit_permission.rs - Converte a string `allow` em `true` e rejeita qualquer outro valor com um erro.
src/agents/id.rs - Valida se um identificador não vazio contém apenas letras ASCII, dígitos, sublinhados ou hífens.
src/agents/load.rs - Carrega e valida um agente a partir de um arquivo Markdown, retornando-o ou informando o motivo da falha.
src/agents/model.rs - Desserializa um modelo opcional e normaliza seu valor, removendo espaços externos e convertendo-o para minúsculas.
src/agents/tree_default.rs - Permite o uso da ferramenta TREE retornando sempre `true`, sem validações ou efeitos colaterais.
src/agents/tree_permission.rs - Desserializa `allow` como `true` e rejeita qualquer outro valor com erro indicando o valor esperado.
src/bin/documentation.rs - Conecta-se ao S.I.R.K. e executa o fluxo de documentação, retornando seu resultado.
src/git_service.rs - Centraliza e expõe as operações de adição ao Git e consulta do status do repositório.
src/git_service/add.rs - Adiciona ao Git todas as alterações do projeto associado ao diretório informado e retorna erros caso a localização ou o comando falhe.
src/git_service/project.rs - Resolve um diretório Git e retorna a raiz do projeto e seu prefixo relativo, convertendo falhas em mensagens de erro.
src/git_service/run.rs - Executa comandos Git no diretório indicado e retorna a saída padrão em bytes, contextualizando falhas de execução ou de status.
src/git_service/status.rs - Lista, em ordem e sem duplicatas, os caminhos alterados no Git dentro do diretório solicitado, excluindo arquivos ignorados.
src/harness.rs - Reexporta os tipos das interfaces que definem os contratos de integração do harness.
src/history.rs - Define o histórico de conversas editável, reunindo seu caminho, snapshot, blocos e arquivo de lock.
src/history/create.rs - Cria e salva um histórico vazio associado ao snapshot, em arquivo identificado pelo horário e PID do processo.
src/history/drop.rs - Libera o bloqueio de `History` ao descartar o valor, ignorando erros ao desbloquear.
src/history/lock.rs - Adquire um bloqueio exclusivo no arquivo de bloqueio associado ao histórico.
src/history/reserved.rs - Identifica linhas que contêm marcadores reservados do histórico, sem produzir efeitos colaterais.
src/history/save.rs - Persiste o snapshot do histórico em disco, serializando metadados e blocos em YAML.
src/http.rs - Expõe a função `serve` do transporte HTTP e organiza seus módulos internos de suporte.
src/http/handle.rs - Encaminha verificações de saúde e solicitações HTTP para operações de agentes e Git, retornando os resultados adequados.
src/http/serve.rs - Inicia um servidor HTTP que processa requisições em threads separadas e retorna respostas JSON.
src/http/tests.rs - Agrupa e organiza os módulos que testam o comportamento HTTP.
src/http/tests/enforces_agent_delete_permissions.rs - Verifica que arquivos só são excluídos quando as permissões explícitas do agente estão liberadas.
src/http/tests/handles_agent_edits.rs - Testa se uma requisição HTTP executa um agente capaz de editar um arquivo e retornar a resposta esperada.
src/http/tests/handles_agent_requests.rs - Verifica via requisição HTTP se a execução de um agente retorna o status e o resultado esperados.
src/http/tests/handles_git_add_requests.rs - O teste confirma que a rota `/v1/git/add` adiciona ao stage do Git um arquivo no diretório informado.
src/http/tests/handles_git_status_requests.rs - Verifica se a rota de status do Git retorna apenas os arquivos visíveis, excluindo os ignorados por `.treeignore`.
src/http/tests/preserves_literal_template_input.rs - Verifica se `{{ outputs.plan }}` é preservado literalmente na entrada do agente enviada pela rota HTTP.
src/http/tests/rejects_invalid_requests.rs - Verifica as respostas HTTP de sucesso e erro para rotas, métodos e corpos inválidos.
src/http/types.rs - Define estruturas para solicitações de agente e diretório, além de respostas HTTP com status e corpo.
src/input.rs - Implementa a leitura de respostas e confirmações do usuário pelo terminal interativo.
src/interfaces/harness.rs - Define a interface e os tipos comuns para integrar adaptadores que executam agentes de programação.
src/interfaces/harness/display.rs - Formata variantes de `HarnessError` como mensagens legíveis, incluindo o adaptador e os detalhes disponíveis.
src/interfaces/history.rs - Define os tipos `Snapshot` e `Block` para representar e serializar registros do histórico.
src/interfaces/history/marker.rs - Retorna o marcador textual fixo correspondente ao tipo de bloco do histórico.
src/interfaces/history/text.rs - Retorna uma referência ao texto associado ao bloco do histórico, sem modificá-lo.
src/interfaces/input.rs - Define a interface comum para perguntar ao usuário e aguardar confirmações, retornando respostas ou erros.
src/interfaces/invocation.rs - Adiciona um comando prefixo à invocação, preservando o programa original e suas configurações de execução.
src/interfaces/mod.rs - Centraliza e reexporta tipos comuns de harness, histórico, entrada do usuário e invocação.
src/lib.rs - Declara os módulos do crate, expondo-os publicamente, exceto `agent_service` e `git_service`, que permanecem privados.
src/main.rs - Encaminha os argumentos do comando para execução e define o código de saída conforme o resultado.
src/main/create_agent.rs - Cria um agente no diretório atual com entrada pelo terminal e exibe o caminho criado.
src/main/http.rs - Inicia o servidor HTTP no endereço informado ou no padrão, retornando eventuais erros.
src/main/print_usage.rs - Exibe na saída padrão o texto de uso fornecido pelo módulo `usage`.
src/main/run.rs - Despacha os argumentos para criar um agente, executar o comando HTTP ou exibir a ajuda.
src/main/tests.rs - Reúne testes de execução e uso da aplicação, com acesso às funções `run` e `usage`.
src/main/tests/accepts_no_arguments_without_starting_a_prompt.rs - Verifica que `run`, chamada sem argumentos, termina sem erro; não descreve um fluxo de produção.
src/main/tests/rejects_an_invalid_command.rs - Confirma que o comando inválido `"invalid"` é rejeitado com uma mensagem de erro correspondente.
src/main/tests/rejects_removed_workflow_commands.rs - Verifica se o comando `run` rejeita comandos inválidos ou removidos com uma mensagem de erro esperada.
src/main/tests/usage_lists_service_commands.rs - O arquivo contém um teste que verifica se `usage()` inclui os comandos esperados e omite os obsoletos.
src/main/usage.rs - Exibe a ajuda de uso do S.I.R.K., com comandos disponíveis, endereço padrão e orientações para criar agentes e cancelar perguntas.
src/services/bash.rs - Define o serviço compartilhado para executar processos via Bash e representar seus status e saídas capturadas.
src/services/bash/default.rs - Configura o BashService padrão para executar comandos usando o executável `bash`.
src/services/bash/execute_bytes_to.rs - Executa uma invocação do Bash e retorna seu status junto aos bytes capturados da saída.
src/services/bash/execute_streaming.rs - Executa a invocação recebida, encaminha sua saída à saída padrão e retorna o resultado ou erro de E/S.
src/services/bash/execute_to.rs - Executa uma invocação, envia o stdout em UTF-8 ao destino, preserva o status e propaga erros.
src/services/bash/new.rs - Constrói um BashService e armazena no campo `executable` o valor informado convertido em String.
src/services/bash/render.rs - Converte uma `Invocation` em uma linha de comando citando o programa e os argumentos conforme o ambiente.
src/services/mod.rs - Reúne e reexporta os módulos e tipos usados para construir e executar comandos.
src/services/quote.rs - Converte `value` em texto protegido para uso em shell, preservando referências a variáveis válidas do ambiente.
src/tools/delete.rs - Remove com segurança um arquivo regular dentro do diretório de execução, validando o caminho e sincronizando o diretório pai.
src/tools/edit.rs - Define os tipos que representam operações de edição, seus parâmetros e o estado anterior dos arquivos.
src/tools/edit/apply_to.rs - Aplica uma edição validada ao conteúdo e retorna o texto atualizado ou um erro.
src/tools/edit/commit.rs - Aplica uma edição pendente ao arquivo de destino, protege contra alterações concorrentes e retorna o diff.
src/tools/edit/diff.rs - Gera um diff unificado entre o conteúdo original e o editado para apresentar as alterações preparadas.
src/tools/edit/display.rs - Renderiza e imprime uma diferença, usando cores quando a saída padrão é terminal e `NO_COLOR` não está definido.
src/tools/edit/pending_validate.rs - Valida a consistência de um registro `Pending` com a requisição, o estado anterior do arquivo e a operação.
src/tools/edit/prepare.rs - Prepara a edição do arquivo, validando a operação e registrando o conteúdo anterior ou sua ausência.
src/tools/edit/read_optional.rs - Lê um arquivo UTF-8 e retorna seu conteúdo, indica sua ausência ou descreve falhas de leitura.
src/tools/edit/render.rs - Renderiza diffs, destacando adições e remoções com cores e escapando caracteres de controle do conteúdo.
src/tools/edit/request_validate.rs - Valida se os campos de uma solicitação de edição são compatíveis com a operação.
src/tools/edit/sync_parent.rs - Sincroniza em disco o diretório pai do caminho informado e converte falhas em mensagens de texto.
src/tools/edit/target.rs - Resolve e valida o caminho de destino de uma edição, garantindo que permaneça dentro do diretório de execução.
src/tools/edit/tests.rs - Cria uma requisição de edição com a operação, a entrada e a versão calculada a partir do conteúdo anterior.
src/tools/edit/tests/append_and_prepend_are_exact_and_versions_are_checked.rs - Valida as operações de anexar e antepor texto, inclusive vazio, e a detecção de conflitos de versão.
src/tools/edit/tests/diff_and_color_rendering_preserve_plain_results.rs - Testa a geração e renderização de diferenças de texto, incluindo cores, escapes e ausência de quebra final.
src/tools/edit/tests/line_edits_preserve_bytes_and_handle_eof.rs - Testa se inserções, exclusões e substituições por linha preservam os bytes e rejeitam linhas fora do intervalo.
src/tools/edit/version.rs - Calcula o SHA-256 dos bytes UTF-8 de uma string e retorna o digest em hexadecimal minúsculo.
src/tools/execute_with_input.rs - Executa TREE ou READ no diretório indicado, formatando listagens ou retornando o conteúdo solicitado.
src/tools/format_paths.rs - Serializa caminhos de arquivos em JSON legível e informa falhas de conversão ou serialização.
src/tools/mod.rs - Organiza as ferramentas disponíveis aos agentes e reexporta as operações de execução e solicitação.
src/tools/new_agent.rs - Cria e salva uma definição de agente validada a partir das respostas do usuário e do adaptador.
src/tools/new_agent/adapter.rs - Solicita ao usuário um adaptador disponível e retorna a escolha válida, repetindo a pergunta se a entrada não corresponder.
src/tools/new_agent/answer.rs - Solicita respostas repetidamente até receber um valor não vazio e propaga erros ocorridos durante a interação.
src/tools/new_agent/create.rs - Gera um agente no diretório informado e retorna o caminho onde foi salvo.
src/tools/read.rs - Organiza os módulos internos da ferramenta de leitura e disponibiliza suas principais funções para uso externo.
src/tools/read/component.rs - Compara um padrão e um valor como sequências de caracteres e retorna se correspondem.
src/tools/read/component_match.rs - Compara sequências de caracteres com suporte a curingas `*` e `?`, indicando se correspondem.
src/tools/read/components.rs - Compara componentes de um padrão e de um caminho, aceitando `**` e correspondência por prefixo.
src/tools/read/enumerate.rs - Prefixa cada linha do conteúdo com numeração iniciada em 1, preservando seus terminadores originais.
src/tools/read/enumerated_content.rs - Reconstrói o conteúdo original de uma saída numerada, removendo os números e concatenando as linhas válidas.
src/tools/read/ignore.rs - Determina se um caminho corresponde ao padrão de ignorados, comparando seus componentes conforme a estrutura do padrão.
src/tools/read/ignored.rs - Verifica se um caminho corresponde às regras de exclusão definidas em `.readignore`.
src/tools/read/run.rs - Lê arquivos UTF-8 dentro do diretório de execução, bloqueando caminhos inválidos ou ignorados.
src/tools/request.rs - Interpreta pedidos de leitura da árvore de arquivos, identificando o tipo e o caminho solicitado.
src/tools/tree.rs - Representa a raiz de uma árvore de arquivos e seus caminhos relativos, ordenados e sem duplicatas.
src/tools/tree/list.rs - Lista em uma árvore os arquivos existentes e não ignorados de um diretório de trabalho Git.
src/tools/tree/path.rs - Converte bytes em `PathBuf`, preservando-os no Unix e exigindo UTF-8 nas demais plataformas.