$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$venvPython = Join-Path $scriptDir ".venv\Scripts\python.exe"
$scriptPath = Join-Path $scriptDir "typejet_cli.py"

if (Test-Path $venvPython) {
    & $venvPython $scriptPath $args
} else {
    & python $scriptPath $args
}
