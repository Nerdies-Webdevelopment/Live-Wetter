param(
    [switch]$Release
)

$ErrorActionPreference = "Stop"

if ($Release) {
    dx build --web --release
    & (Join-Path $PSScriptRoot "sync-pwa-assets.ps1") -Release
} else {
    dx build --web
    & (Join-Path $PSScriptRoot "sync-pwa-assets.ps1")
}
