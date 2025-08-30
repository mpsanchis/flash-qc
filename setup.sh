uvx pre-commit install 2>/dev/null || pipx run pre-commit install 2>/dev/null || { echo "Error: Neither uvx nor pipx is installed"; exit 1; }
