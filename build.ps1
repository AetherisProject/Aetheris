# Aetheris Build Script
param(
    [ValidateSet("debug", "release")]
    [string]$Profile = "release",
    [switch]$Clippy,
    [switch]$Test,
    [switch]$Fmt,
    [switch]$Doc,
    [switch]$All
)

if ($All) { $Clippy = $Test = $Fmt = $Doc = $true }

$jobs = 2  # Avoid proc-macro DLL collision on Windows MSVC

Write-Host "Building Aetheris ($Profile profile)..." -ForegroundColor Cyan

# Format check
if ($Fmt) {
    Write-Host "Checking formatting..." -ForegroundColor Yellow
    cargo fmt -- --check
    if ($LASTEXITCODE -ne 0) { throw "Format check failed" }
}

# Clippy
if ($Clippy) {
    Write-Host "Running clippy..." -ForegroundColor Yellow
    cargo clippy --all-features --all-targets -- -D warnings
    if ($LASTEXITCODE -ne 0) { throw "Clippy failed" }
}

# Build
Write-Host "Building..." -ForegroundColor Yellow
$flag = if ($Profile -eq "release") { "--release" } else { "" }
$env:CARGO_BUILD_JOBS = $jobs
cargo build $flag --features full
if ($LASTEXITCODE -ne 0) { throw "Build failed" }

# Test
if ($Test) {
    Write-Host "Running tests..." -ForegroundColor Yellow
    $env:CARGO_BUILD_JOBS = $jobs
    cargo test --all-features
    if ($LASTEXITCODE -ne 0) { throw "Tests failed" }
}

# Doc
if ($Doc) {
    Write-Host "Building docs..." -ForegroundColor Yellow
    cargo doc --no-deps --all-features
    if ($LASTEXITCODE -ne 0) { throw "Doc build failed" }
}

Write-Host "Build complete!" -ForegroundColor Green
