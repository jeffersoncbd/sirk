#!/usr/bin/env bash

set -euo pipefail

# 1. Valida se o parâmetro foi informado
if [ -z "${1:-}" ]; then
  echo "Erro: Forneça o caminho de um arquivo como primeiro parâmetro." >&2
  exit 1
fi

FILE_PATH="$1"

# 2. Remove a extensão (qualquer texto após o último ponto no nome do arquivo)
PATH_WITHOUT_EXT="${FILE_PATH%.*}"

# 3. Adiciona o prefixo "doc/" e altera a extensão para ".md"
DOC_PATH="docs/${PATH_WITHOUT_EXT}"

# 4. Exibe no stdout
echo "${DOC_PATH}.md"
