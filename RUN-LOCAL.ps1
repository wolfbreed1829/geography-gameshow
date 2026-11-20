$ErrorActionPreference = 'Stop'
Set-Location -LiteralPath $PSScriptRoot
New-Item -ItemType Directory -Force -Path 'test-results', 'playwright-report' | Out-Null
$env:NO_COLOR = 'true'
trunk serve --locked --skip-version-check --port 8090 --address 127.0.0.1 --ignore test-results --ignore playwright-report --ignore node_modules
