# Aetheris CLI wrapper for PowerShell
$cmd = Get-Command aeth -ErrorAction SilentlyContinue
if (-not $cmd) {
    Write-Error "aeth not found in PATH. Please install Aetheris first."
    exit 1
}
& aeth @args
