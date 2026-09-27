.env.example - Arquivo .env de exemplo que documenta as variáveis OLLAMA_WEB_URL, OLLAMA_API_KEY, OPENROUTER_URL e OPENROUTER_API_KEY obrigatórias.
Cargo.toml - O manifesto configura o pacote Rust `new-harness` como biblioteca e CLI neutro de provedor, com dependências para serialização, ambiente, hashes e comparação textual.
src/adapters/codex.rs - Converte um RunRequest em uma Invocation de linha de comando para o harness `codex exec`, sempre em modo somente leitura e não interativo.
src/adapters/codex/default.rs - Implementação padrão do `CodexAdapter`, que instancia o adaptador com o identificador "codex" via `CodexAdapter::new("codex")`.
src/adapters/codex/new.rs - Cria um CodexAdapter configurado com o caminho de um executável, sem validações ou falhas.
src/adapters/mod.rs - Módulo que declara e reexporta os adaptadores de provedores de LLM (Codex, Ollama, OpenCode, OpenRouter), a função `resolve` e a lista `AVAILABLE`.
src/adapters/ollama.rs - Traduz requisições neutras de provider em invocações da CLI do Ollama (`ollama run <modelo> <prompt>`), validando modelo e opções.
src/adapters/ollama/default.rs - Fornece a implementação padrão do `OllamaAdapter`, criando a instância via `OllamaAdapter::new("ollama")`.
src/adapters/ollama/new.rs - Cria uma instância do adaptador Ollama armazenando o caminho do executável informado, sem validações.
src/adapters/ollama_web.rs - Traduz requisições do harness em chamadas HTTP via curl à API do Ollama, injetando a chave com segurança e extraindo o texto da resposta.
src/adapters/ollama_web/api_key.rs - Resolve a chave de API do Ollama Web, priorizando valor explícito, variável de ambiente e, por fim, arquivo `.env`.
src/adapters/ollama_web/default.rs - Valores padrão do OllamaWebAdapter: executável curl, sem URL base nem chave de API.
src/adapters/ollama_web/dotenv.rs - Lê uma chave específica do arquivo .env de um diretório e retorna seu valor não vazio, se existir.
src/adapters/ollama_web/endpoint.rs - Resolve a URL completa do endpoint de geração do Ollama Web a partir de configuração, variável de ambiente ou arquivo .env.
src/adapters/ollama_web/new.rs - Constrói um OllamaWebAdapter normalizando executable, base URL não vazia e api_key opcional.
src/adapters/ollama_web/nonempty.rs - Valida se uma String possui texto não em branco, retornando-a como Option.
src/adapters/opencode.rs - Adapta requisições neutras para comandos da CLI OpenCode, preservando opções e diretório de trabalho.
src/adapters/opencode/default.rs - Implementa `Default` para `OpenCodeAdapter`, construindo-o via `Self::new("opencode")`.
src/adapters/opencode/id.rs - A macro define o método que retorna o identificador estático `"opencode"` para o adaptador.
src/adapters/opencode/invocation.rs - Monta a invocação do OpenCode para executar uma solicitação do harness com as opções configuradas e o prompt fornecido.
src/adapters/opencode/new.rs - Constrói um `OpenCodeAdapter` convertendo o caminho do executável em `String` e armazenando-o no campo `executable`.
src/adapters/opencode/tests.rs - Organiza os testes do adaptador OpenCode, reunindo módulos e importações sem lógica de execução própria.
src/adapters/opencode/tests/does_not_enable_automatic_permission_approval.rs - Garante que a invocação do OpenCode não habilite a aprovação automática de permissões.
src/adapters/opencode/tests/translates_generic_options_to_opencode_run.rs - Verifica se opções genéricas de execução são convertidas corretamente no comando `opencode run`, com argumentos, diretório e ambiente esperados.
src/adapters/openrouter.rs - Adapta requisições do harness em chamadas curl à API de chat do OpenRouter, sem streaming.
src/adapters/openrouter/api_key.rs - Resolves a chave de API do OpenRouter via campo interno, variável de ambiente ou arquivo .env.
src/adapters/openrouter/default.rs - Define o impl Default do OpenRouterAdapter, com executable "curl" e base_url e api_key vazios.
src/adapters/openrouter/dotenv.rs - Lê o arquivo .env de um diretório e retorna o valor de uma variável específica, se presente e não vazia.
src/adapters/openrouter/endpoint.rs - Resolve a URL do endpoint de chat da OpenRouter via base_url, variável de ambiente ou .env, com fallback padrão.
src/adapters/openrouter/new.rs - Constrói um OpenRouterAdapter, normalizando a URL base (não vazia via nonempty) e descartando api_key ausente ou em branco.
src/adapters/openrouter/nonempty.rs - Valida se uma string não está vazia após trim e a retorna como Option<String>.
src/adapters/resolve.rs - Converte um nome de harness em `Box<dyn HarnessAdapter>`, retornando `None` para nomes desconhecidos.
src/agents.rs - Converte um arquivo Markdown de agente com front matter YAML em um `Agent` validado.
src/agents/call_prefix.rs - Desserializa o campo `call_prefix` de configuração de agente, aceitando string ou lista e validando tokens não vazios.
src/agents/delete_permission.rs - Deserializador customizado que converte a string "allow" em bool para a tool de delete, validando a configuração.
src/agents/edit_permission.rs - Desserializador de `bool` que só aceita o valor "allow" e retorna `true`, senão erro.
src/agents/id.rs - Valida se um &str é um identificador não vazio com apenas caracteres alfanuméricos ASCII, _ ou -.
src/agents/load.rs - Carrega um agente de um arquivo Markdown no diretório, validando o ID e convertendo falhas em mensagem de erro.
src/agents/model.rs - Normaliza o campo `model` de struct para `Option<String>` em minúsculas e sem espaços.
src/agents/tree_default.rs - Função que autoriza sempre o uso da ferramenta TREE, retornando `true` incondicionalmente.
src/agents/tree_permission.rs - Deserializador customizado que converte a string TREE_TOOL "allow" em booleano true, falhando com erro descritivo caso contrário.
src/harness.rs - Fachada que reexporta HarnessAdapter, HarnessError, Invocation e RunRequest de `crate::interfaces`.
src/history.rs - Define `History` para persistir e recuperar o transcript editável da execução, mantendo acesso exclusivo ao arquivo.
src/history/create.rs - Cria e persiste uma instância de History com arquivo de log único e bloqueado no diretório do Snapshot.
src/history/drop.rs - Libera o lock do History ao descartá-lo, ignorando falhas de unlock.
src/history/finish.rs - Fecha o bloco em construção, converte-o em um Block tipado e o adiciona ao último passo.
src/history/lock.rs - Abre o arquivo de lock do histórico e adquire bloqueio exclusivo, sinalizando erros via String.
src/history/open.rs - Abre, bloqueia e carrega um arquivo de histórico, parseando seu conteúdo para retornar a struct History preenchida.
src/history/parse.rs - Interpreta um histórico v2, desserializa o snapshot e organiza o conteúdo em etapas, blocos e rótulos.
src/history/reserved.rs - Função `reserved` indica se uma linha do log é um marcador de seção reservada.
src/history/save.rs - Serializa o histórico de uma workflow em YAML legível, com gravação atômica em arquivo temporário.
src/input.rs - Fornece entrada de usuário via terminal, com `ask` lendo respostas do stdin e `await_confirmation` lendo de `/dev/tty`.
src/interfaces/harness.rs - Converte uma RunRequest em uma Invocation para o CLI do agente, validando as opções suportadas.
src/interfaces/harness/display.rs - Implementa a trait `Display` para `HarnessError`, formatando cada variante como mensagem legível.
src/interfaces/history.rs - Define os tipos de dados do histórico: o struct `Snapshot` serializável e o enum `Block` de tipos de bloco.
src/interfaces/history/marker.rs - Retorna o marcador de seta textual associado a cada variante do enum Block, indicando entrada ou saída de dados.
src/interfaces/history/text.rs - Fornece o texto (`&str`) contido em qualquer variante de `Block` via match exaustivo.
src/interfaces/input.rs - Define o contrato de interação com o usuário via `ask`, com `await_confirmation` derivado, retornando `Result<String, String>`.
src/interfaces/invocation.rs - Aplica um prefixo a uma Invocation, redefinindo o programa e antepondo argumentos.
src/interfaces/mod.rs - Módulo raiz de interfaces que reexporta os tipos compartilhados entre as sub-rotinas internas.
src/interfaces/workflow.rs - Estruturas serializáveis (serde) que definem um workflow, seus passos, entradas e coordenadas de edição.
src/interfaces/workflow/default.rs - Implementa `Default` para `StepInput`, criando uma entrada de texto vazia como valor inicial.
src/lib.rs - Arquivo raiz do crate que apenas declara e expõe publicamente os dez módulos da biblioteca.
src/main.rs - Ponto de entrada do CLI que repassa argumentos da linha de comando ao módulo run e retorna o código de saída do processo.
src/main/create_agent.rs - Cria um agente no diretório atual usando entrada de terminal e exibe o caminho criado.
src/main/print_usage.rs - Exibe na saída padrão o texto de uso retornado por `super::usage::usage()`.
src/main/resume.rs - Retoma a execução do harness a partir de um caminho de estado, propagando erros e concluindo com sucesso.
src/main/run.rs - Interpreta os argumentos de linha de comando e delega ao módulo correspondente: criar agente, executar workflow, retomar execução ou exibir ajuda.
src/main/run_workflow.rs - Carrega um workflow pelo nome, resolve o diretório atual e inicia sua execução, propagando erros.
src/main/tests.rs - Valida a CLI, seu texto de uso e a resolução segura de nomes de fluxos em `flows/`.
src/main/tests/accepts_no_arguments_without_starting_a_prompt.rs - Confirma que a execução sem argumentos termina sem erro, sem verificar se um prompt foi iniciado.
src/main/tests/rejects_an_invalid_command.rs - Confirma que `run` rejeita comandos inválidos com uma mensagem de erro apropriada.
src/main/tests/rejects_flow_paths_and_extensions.rs - Confirma que `workflow_path` rejeita caminhos vazios, sem extensão, com travessia de diretório ou aninhados.
src/main/tests/resolves_flow_names_inside_the_flows_directory.rs - Verifica se nomes de fluxo são resolvidos para os arquivos `.yml` esperados em `flows/`.
src/main/tests/usage_lists_explicit_flow_commands.rs - Verifica se a saída de `usage()` descreve comandos de fluxo e omite comandos interativos.
src/main/usage.rs - Retorna uma string estática com as instruções de uso do CLI para criar agentes e executar ou retomar fluxos.
src/main/workflow_path.rs - Valida o nome de um fluxo e constrói o caminho `flows/<nome>.yml`.
src/runner/all_steps.rs - Coleta recursivamente todos os Step aninhados de uma lista, aplainando os ramos iter, is_true e is_false.
src/runner/execute.rs - Executa um comando via BashService com streaming e retorna a saída padrão, ou erro se o processo falhar.
src/runner/execution.rs - Avança um passo do workflow, executando tools e persistindo o histórico do agente.
src/runner/execution/continue_with.rs - Retoma um workflow a partir do snapshot, validando-o e executando os passos, retornando os outputs por passo.
src/runner/execution/run_steps.rs - Engine que executa os passos do workflow, resolvendo loops e condicionais recursivamente e persistindo resultados no histórico.
src/runner/external.rs - Agrega os submódulos de parsing das ferramentas externas EDIT/DELETE e define o ExternalDeleteRequest com mensagens de erro.
src/runner/external/completed_delete.rs - Verifica se um bloco Delete de requisição externa já foi concluído no histórico.
src/runner/external/completed_edit.rs - Verifica no histórico de blocos se um Request de edição já foi executado com sucesso anteriormente.
src/runner/external/delete_pending.rs - Detecta pedido de exclusão confirmado no fim do histórico e valida a entrada pendente.
src/runner/external/delete_request.rs - Interpreta o comando de texto `DELETE:<json>` e o converte em um `ExternalDeleteRequest` desserializado.
src/runner/external/edit_pending.rs - Detecta par Output/Input final no histórico e retorna o Pending correspondente via validação e desserialização.
src/runner/external/edit_request.rs - Analisa um comando `EDIT:` textual e o converte em um `Request` de edição validado via JSON.
src/runner/external/prepare_edit.rs - Valida limites de linha de um arquivo e prepara a edição pendente com o hash do conteúdo lido, sem escrevê-lo ao disco.
src/runner/external/tool_result.rs - Converte o resultado de uma ferramenta em um Block de histórico, mapeando "TREE" para Block::Tree e demais ferramentas para Block::Read.
src/runner/external/user_tool_request.rs - Detecta se o último bloco do histórico é resposta a uma pergunta de tool, retornando nome e argumentos da tool.
src/runner/external/validate_user_tool_result.rs - Valida se o bloco atual do histórico corresponde à ferramenta solicitada e avança a posição em caso de match.
src/runner/history_validation.rs - Valida estruturalmente blocos de um transcript persistido, garantindo que cada passo do workflow tenha histórico correspondente.
src/runner/history_validation/agent_blocks.rs - Valida a sequência de blocos de um turno, garantindo coerência entre pedidos de ferramentas e as respostas do usuário.
src/runner/history_validation/ask_blocks.rs - Valida se o histórico de blocos forma um par ASK/Input válido, indicando se o processamento deve ser refeito.
src/runner/history_validation/edit_blocks.rs - Valida entradas de histórico do tipo EDIT, confirmando se o Pending foi aplicado com o diff de saída esperado.
src/runner/history_validation/step_blocks.rs - Valida a sequência de blocos de uma etapa da conversa, garantindo ordem e estado corretos.
src/runner/history_validation/visit.rs - Valida os passos do workflow contra o histórico gravado, conferindo blocos e labels para verificar se o histórico cobre todos os passos.
src/runner/mod.rs - Facade pública do runner, reexportando `run`, `run_with`, `run_interactive_with`, `resume` e `continue_with` como API estável de execução e retomada de workflows.
src/runner/question.rs - Extrai a pergunta de uma linha de comando iniciada por `ASK:`, retornando-a aparada ou `None` se o prefixo não existir.
src/runner/resume.rs - Retoma uma execução pausada a partir do histórico persistido, executando os comandos restantes via TerminalInput.
src/runner/run.rs - Executa um Workflow interativamente via TerminalInput e retorna as variáveis de saída.
src/runner/run_interactive.rs - Prepara e executa o workflow interativo, retornando um mapa de resultados por passo.
src/runner/run_with.rs - Executa um `Workflow` de forma não interativa, usando `NoInput` como provedor de entrada.
src/runner/run_with/no_input.rs - Estrutura `NoInput` que implementa `UserInput` falhando imediatamente, bloqueando fluxos que exigem interação do usuário.
src/runner/tests.rs - Módulo de fixtures de teste do runner: cria projetos temporários com git e helpers, removendo tudo ao final.
src/runner/tests/control_flow.rs - Suíte de testes do runner que valida os passos LOOP, IF e WRITE e a validação do histórico.
src/runner/tests/conversations.rs - Testes que validam o protocolo de conversas do runner: chamadas a agentes, perguntas, ferramentas e retomada de execuções.
src/runner/tests/edits.rs - Módulo de testes do runner que valida edição por coordenadas, ferramentas do agente e retomada de execuções.
src/runner/tests/files.rs - Suíte de testes do runner que valida as ferramentas EDIT/WRITE/DELETE, versionamento e contenção de caminhos.
src/runner/validate_snapshot.rs - Valida se um snapshot de execução persistido está íntegro e pode ser retomado com segurança.
src/services/bash.rs - Serviço compartilhado que executa comandos via `bash -lc` com argumentos escapados e captura saída e status de saída.
src/services/bash/default.rs - Implementa `Default` para `BashService`, criando uma instância com o executável "bash".
src/services/bash/execute_bytes_to.rs - Executa um comando Bash de uma Invocation e captura o stdout bruto, espelhando-o em um Write.
src/services/bash/execute_streaming.rs - Executa um Invocation no shell, redirecionando stdout para o processo atual e retornando o ProcessOutput.
src/services/bash/execute_to.rs - Executa um comando bash, escrevendo o stdout em tempo real no destino informado e retornando o ProcessOutput.
src/services/bash/new.rs - Constrói um `BashService` armazenando o caminho do executável shell a ser usado nos comandos.
src/services/bash/render.rs - Monta a string de comando shell de uma Invocation, prefixando `exec --` e aplicando shell_quote em programa e argumentos.
src/services/mod.rs - Módulo raiz de serviços que registra o submódulo bash e reexporta Invocation, BashService e ProcessOutput.
src/services/quote.rs - Converte um valor arbitrário em string segura para interpolação em comandos shell POSIX.
src/tools/custom/args.rs - Converte uma string JSON em Vec<String>, retornando erro se a entrada não for uma lista de strings.
src/tools/custom/mod.rs - Agrega os módulos de argumentos, nome e execução da ferramenta `custom-tool`, expondo `arguments`, `valid_name` e `execute` em um único ponto de importação.
src/tools/custom/name.rs - Valida se um nome não é vazio e contém apenas caracteres alfanuméricos ASCII, `_` ou `-`.
src/tools/custom/run.rs - Executa um script Bash em tools/ com nome e caminhos validados, devolvendo seu stdout ou erro CUSTOM-TOOL.
src/tools/delete.rs - Remove com segurança um arquivo comum dentro do diretório de execução, bloqueando travessia, symlinks e caminhos fora da raiz.
src/tools/edit.rs - Expõe operações de edição de arquivo (insert, delete, replace, prepend, append) com verificação de versão e geração de diff.
src/tools/edit/apply_to.rs - Aplica uma operação de edição (Append/Prepend/Insert/Delete/Replace) sobre um texto com verificação de versão.
src/tools/edit/commit.rs - Aplica a edição preparada de forma atômica, detectando alterações concorrentes e retornando o diff resultante.
src/tools/edit/diff.rs - Gera um diff unificado entre o texto original e o resultado da aplicação de uma edição pendente.
src/tools/edit/display.rs - Exibe o diff renderizado no stdout, usando cores apenas quando a saída é um terminal real.
src/tools/edit/pending_validate.rs - Valida um registro de edição pendente, exigindo criação apenas para arquivos ausentes com Append/Prepend.
src/tools/edit/prepare.rs - Prepara uma edição validando o arquivo alvo e guardando seu conteúdo anterior no Pending.
src/tools/edit/read_optional.rs - Lê o conteúdo de um arquivo opcional como texto UTF-8, retornando None se ele não existir e Err em caso de falha.
src/tools/edit/render.rs - Renderiza um diff de texto, colorindo linhas adicionadas (verde) e removidas (vermelho) em hunks.
src/tools/edit/request_validate.rs - Valida se um pedido de edição possui campos obrigatórios e coordenadas compatíveis com a operação.
src/tools/edit/sync_parent.rs - Sincroniza o diretório pai de um caminho, forçando a gravação de seus metadados no disco via fsync.
src/tools/edit/target.rs - Resolve e valida o caminho de um arquivo dentro do diretório de execução.
src/tools/edit/version.rs - Gera o hash SHA-256 hexadecimal de um texto a partir de uma string `&str`, sem validações ou efeitos colaterais.
src/tools/execute.rs - Executa a ferramenta `name` no diretório indicado, sem entrada, propagando o resultado.
src/tools/execute_with_input.rs - Despacha a ferramenta `name` (TREE, GIT-STATUS-TREE ou READ) no diretório informado, somente para leitura.
src/tools/format_paths.rs - Converte uma lista de caminhos de arquivos em um array JSON formatado com quebras de linha.
src/tools/git_status_tree.rs - Estrutura de dados que guarda a raiz do repositório e a lista de caminhos alterados pelo git status.
src/tools/git_status_tree/git.rs - Executa um comando git no diretório raiz e retorna seu ExitStatus com os bytes de stdout.
src/tools/git_status_tree/list.rs - Retorna caminhos de arquivos modificados, adicionados, excluídos, renomeados, copiados, mesclados e não rastreados em um repositório Git.
src/tools/git_status_tree/path.rs - Converte bytes crus de um caminho do Git em PathBuf, validando UTF-8 fora de plataformas Unix.
src/tools/git_status_tree/tree_ignored.rs - Coleta caminhos ignorados pelo `.treeignore` via `git ls-files`, retornando-os deduplicados e ordenados em um conjunto.
src/tools/mod.rs - Agrava e reexporta os módulos de despacho de ferramentas da CLI, expondo as funções de execução e ocultando as de uso interno.
src/tools/new_agent.rs - Cria interativamente um arquivo de agente Markdown em `.agents/` via LLM, validando-o antes de gravar.
src/tools/new_agent/adapter.rs - Solicita ao usuário um adaptador válido, reenviando a pergunta até a entrada corresponder a um item de `adapters::AVAILABLE`.
src/tools/new_agent/answer.rs - Pede ao usuário uma resposta em loop até que o texto digitado não esteja em branco.
src/tools/new_agent/create.rs - Cria um agente em um diretório via comando shell, delegando a `create_with` e retornando o `PathBuf` do agente salvo.
src/tools/read.rs - Expõe a ferramenta `read`, reexportando seu ponto de entrada e mantendo a lógica interna encapsulada nos submódulos.
src/tools/read/component.rs - Verifica se um valor corresponde a um padrão de componente, delegando a comparação.
src/tools/read/component_match.rs - Função pura que verifica recursivamente se um padrão de curingas (`*`, `?`, literais) casa com um valor, ignorando sobras.
src/tools/read/components.rs - Componentes de um glob, com suporte a `**` e correspondência parcial via `prefix`.
src/tools/read/enumerate.rs - Gera uma string numerando cada linha do conteúdo, prefixando índice e conteúdo sob o cabeçalho "Line | Content".
src/tools/read/enumerated_content.rs - Reconstrói o texto original a partir da saída numerada de `enumerate` em um transcript, validando cabeçalho e sequência.
src/tools/read/ignore.rs - Verifica se um caminho corresponde a um padrão glob de ignore de arquivo ou diretório.
src/tools/read/ignored.rs - Verifica se um arquivo sob `root` é ignorado pelo arquivo `.readignore`.
src/tools/read/run.rs - Lê o conteúdo de um arquivo de texto UTF-8, bloqueando caminhos ignorados ou fora do diretório raiz.
src/tools/request.rs - Interpreta uma mensagem de texto como comando de controle, retornando o par TREE ou READ com o caminho, ou None se não for comando isolado.
src/tools/supports.rs - Verifica se um nome de ferramenta corresponde a uma das oito operações permitidas.
src/tools/tree.rs - Representa o inventário recursivo de arquivos de um repositório, respeitando as regras de ignore e índice do Git.
src/tools/tree/list.rs - Tree::list lista arquivos de um repositório Git aplicando filtros .gitignore e .treeignore.
src/tools/tree/path.rs - Converte bytes brutos de caminhos em um PathBuf válido para a plataforma atual.
src/tools/write/directory.rs - Garante que o diretório pai exista, criando-o se necessário, ou retorna erro contextualizado com "WRITE" e o caminho.
src/tools/write/mod.rs - Expõe a API pública de escrita de arquivos, reexportando `write` e `write_with_options` para uso externo.
src/tools/write/run.rs - Grava arquivo no diretório indicado repassando argumentos a `write_with_options` com `false`.
src/tools/write/run_with.rs - Grava um arquivo de texto no diretório de execução, criando diretórios e impedindo escape da raiz.
src/tools/write/run_with/tests.rs - Grava um arquivo em disco no caminho indicado, criando diretórios e exigindo force para sobrescrever.
src/workflow.rs - Valida estaticamente os passos de um workflow, validando campos por tipo, escopo e referências.
src/workflow/condition.rs - Converte texto em bool, aceitando apenas "true" ou "false" após trim.
src/workflow/coordinate_error.rs - Gera a mensagem de erro "EDIT {name} must render to a positive integer" para coordenadas de workflow inválidas.
src/workflow/coordinate_render.rs - Renderiza coordenadas de edição literais ou de template e valida que resultem em inteiros positivos.
src/workflow/coordinate_validation.rs - Renderiza a coordenada de edição, devolvendo 1 linha, usize::MAX para blocos "end" ou o resultado de self.render.
src/workflow/edit_request.rs - Atalho que delega a `edit_request_with` com a flag `false`, repassando outputs e locals.
src/workflow/edit_request_with.rs - Constrói um `Request` de edição a partir de um `Step`, renderizando caminho, coordenadas e versão no escopo atual.
src/workflow/enumerate.rs - Retorna um booleano indicando se a etapa está habilitada para enumeração, com padrão falso.
src/workflow/force.rs - Retorna se a etapa deve ser executada de forma forçada, com `false` como padrão.
src/workflow/from_file.rs - Carrega um Workflow de um arquivo YAML, desserializando seu conteúdo e validando-o antes de retornar.
src/workflow/input_render.rs - Converte um StepInput em String, aplicando os escopos outputs e locals via render_scoped ou serialização JSON.
src/workflow/input_text.rs - Fornece um acessor que retorna o conteúdo de um StepInput quando a entrada é do tipo texto, ou None caso contrário.
src/workflow/is_tool_step.rs - Verifica se um Step do workflow executa alguma ferramenta (tool ou custom_tool).
src/workflow/loop_items.rs - Converte uma string JSON em Vec<String>, exigindo um array de textos e retornando erro descritivo em caso de falha.
src/workflow/loop_target.rs - Extrai o nome de um alvo `{{ loop.NOME }}` da saída, retornando-o apenas se válido.
src/workflow/name.rs - Retorna o nome da etapa (agent, tool ou custom_tool) a partir do Step, ou "invalid" se todos forem None.
src/workflow/output_name.rs - Valida se um nome de saída de workflow contém apenas caracteres alfanuméricos ASCII, `_` ou `-`.
src/workflow/positive_coordinate.rs - Valida que uma coordenada seja maior que zero, devolvendo o valor ou um erro descritivo.
src/workflow/render_input.rs - Renderiza um template com um mapa de saídas delegando a `super::render_scoped` sem escopo.
src/workflow/render_path.rs - Valida se o Step define um path e delega sua renderização com escopo de variáveis.
src/workflow/render_scoped.rs - Substitui marcadores `{{ outputs.* }}` e `{{ loop.* }}` de um template pelos valores correspondentes.
src/workflow/step_branch.rs - Escolhe entre os ramos verdadeiro ou falso de um Step e retorna o rótulo com seus passos.
src/workflow/step_input.rs - Expõe a renderização do input de um Step, delegando ao campo input com outputs ordenados e locals opcionais.
src/workflow/validate.rs - Valida um Workflow checando versão e presença de passos antes de delegar a validate_steps.
