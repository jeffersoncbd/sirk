## Spec: Servidor Web para o CLI (Porta de Health/Hello "NewHarness")

### 1. Contexto & Objetivo
- **Descrição:** Adicionar ao binário do CLI a capacidade de iniciar um servidor HTTP local que, ao ser acessado pelo navegador, exibe na central da página o texto "NewHarness". O servidor deve iniciar somente quando invocado explicitamente por flag/comando, permanecendo inativo no uso normal do CLI.
- **Objetivo:** Fornecer um ponto de entrada HTTP mínimo para verificação de que o processo do CLI está vivo e Correctamente inicializado,useful como base para futures endpoints de diagnóstico, métricas ou inspeção de estado. O texto "NewHarness" serve como marcador visual de que a requisição chegou ao servidor.

### 2. Escopo
#### Incluído (In-Scope)
- [ ] Nova subcomando ou flag (ex.: `serve`, `--serve`, `--port <n>`) que inicia o servidor.
- [ ] Bind em `127.0.0.1` por padrão (localhost), com porta padrão fixa ou configurável.
- [ ]Uma única rota (ex.: `GET /`) que responde com uma página HTML mínima contendo "NewHarness" centralizado.
- [ ] Endpoint de health (ex.: `GET /health`) retornando status simples — opcional, mas recomendado para o objetivo de verificação.
- [ ] Mensagem de log clara no terminal informando URL base, porta em uso e portas alternativas.
- [ ] Shutdown gracioso (Ctrl+C / SIGINT / SIGTERM) fechando sockets e liberando a porta.
- [ ] Tratamento de porta já ocupada com mensagem de erro acionável e código de saída distinto.

#### Não Incluído (Out-of-Scope)
- [ ] Autenticação, tokens, CORS ou qualquer exposição em rede pública (`0.0.0.0`).
- [ ] Endpoints de negócio, APIs REST, WebSockets ou persistência.
- [ ] Bundlers, dependências front-end, frameworks SPA, assets estáticos ou hot-reload.
- [ ] Modificação de comandos existentes do CLI ou refatoração de arquitetura não relacionada.
- [ ] TLS/HTTPS, proxies, deploy em containers ou orquestração.
- [ ] Métricas, tracing, telemetria e logging estruturado avançado.
- [ ] Integração com o restante do domínio do projeto (o texto "NewHarness" é literale literário, sem lógica de negócio).

### 3. Requisitos Funcionais
- [ ] Regra 1: O servidor só é iniciado mediante invocação explícita (ex.: `cli serve --port 8080`); nenhum comando existente deve abrir portas como efeito colateral.
- [ ] Regra 2: Após a inicialização bem-sucedida, o processo deve permanecer em execução (foreground, bloqueante) até receber sinal de interrupção.
- [ ] Regra 3: `GET /` responde com `200 OK` e `Content-Type: text/html; charset=utf-8` com corpo HTML que exiba **exatamente** o texto "NewHarness" centralizado horizontal e verticalmente.
- [ ] Regra 4: O layout deve ser mínimo, sem dependências externas (CSS inline ou bloco `<style>`), responsivo e legível em qualquer viewport.
- [ ] Regra 5: A rota raiz deve responder também a `HEAD` (sem corpo) e a qualquer subpath desconhecido deve retornar `404` (ou redirecionar à raiz, à escolha — documentar a decisão).
- [ ] Regra 6: Métodos não suportados (ex.: `POST /`) devem retornar `405 Method Not Allowed` com cabeçalho `Allow` apropriado.
- [ ] Regra 7: O bind deve restringir-se a loopback por padrão; habilitar `0.0.0.0` apenas mediante flag explícita e opcional, com aviso no terminal.
- [ ] Regra 8: Porta padrão definida como constante; se ocupada, deve-se sugerir a próxima porta livre (comportamento configurável) ou falhar com mensagem clara.
- [ ] Validação de entrada: `--port` deve ser inteiro entre 1 e 65535; valores inválidos geram erro de uso e código de saída ≠ 0, sem stack trace.
- [ ] Tratamento de erro: falha ao abrir socket → mensagem clara no stderr + saída com código 1; erro de renderização/exceção inesperada por requisição → log no terminal e resposta 500 genérica, sem vazar stack trace ao cliente.
- [ ] Tratamento de erro: requisição acima de limite de tempo configurável deve ser encerrada sem derrubar o processo.
- [ ] Regra 9: Ao receber `SIGINT`/`SIGTERM`, o servidor deve parar de aceitar conexões, drenar requisições em curso (com timeout curto) e encerrar com código 0.

### 4. Diretrizes Técnicas
- **Arquivos/Pastas afetadas:** usar a raiz de comandos do CLI como ponto de entrada (ex.: `src/commands/serve.*`), um módulo isolado de servidor (ex.: `src/server/http-server.*`), um handler de rota/view (ex.: `src/server/routes/health.*` ou `src/views/hello.*`) e testes correspondentes em `tests/` ou `__tests__/` (alinhado à convenção existente). Nomes de pastas exatos devem seguir a estrutura já presente no repositório.
- **Convenções:** manter o modelo de "um export por arquivo" e o mesmo estilo de exports/imports do resto do projeto. Preferir a biblioteca HTTP já presente nas dependências (ou o stdlib da linguagem) — **não adicionar dependência nova** sem necessidade; se a stack usar apenas stdlib, usar stdlib. Reutilizar o mecanismo já adotado para parsing de flags/args, logging e códigos de saída. Preservar a ordem de precedência de flags e o formato das mensagens de `--help`.
- **Separação de responsabilidades:** o comando deve apenas orquestrar (parse args, iniciar, aguardar sinal); a criação do socket e o roteamento ficam no módulo de servidor,_testáveis de forma isolada; a string "NewHarness" fica em uma constante única, fácil de localizar e alterar.
- **Testabilidade:** a função de construção do app/handler deve ser separada do `listen`, permitindo testes sem abrir socket real (porta 0 / mock do client HTTP).
- **Documentação:** atualizar README/help do CLI com o novo comando, porta padrão, exemplos de `curl` e nota de segurança (uso apenas em localhost).

### 5. Critérios de Aceite & Validação
- [ ] O comando de teste `[ex: npm run test]` passa sem erros.
- [ ] O comando de lint/format `[ex: npm run lint]` passa sem erros.
- [ ] O type-check `[ex: npm run typecheck]` passa sem erros.
- [ ] Subir o servidor e abrir `http://127.0.0.1:<porta>/` no navegador exibe "NewHarness" centralizado, sem scrollbars e sem dependências externas.
- [ ] `curl -i http://127.0.0.1:<porta>/` retorna `200`, `Content-Type: text/html; charset=utf-8` e o texto "NewHarness" presente no corpo.
- [ ] `curl -i -X POST http://127.0.0.1:<porta>/` retorna `405`; rota inexistente retorna `404`.
- [ ] Nenhum comando existente do CLI passa a abrir socket (verificado inspecionando logs/processos).
- [ ] Porta já ocupada produz mensagem de erro clara e código de saída ≠ 0, sem stack trace.
- [ ] `--port 0` / `--port 99999` / `--port abc` falham com erro de uso e código de saída ≠ 0.
- [ ] Servidor não responde a partir de outro host da rede por padrão (bind em loopback).
- [ ] `Ctrl+C` encerra o processo com código 0 e a porta é liberada (re-executável imediatamente em seguida).
