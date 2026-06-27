param(
    [string]$BasePath = "/Live-Wetter/"
)

$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $MyInvocation.MyCommand.Path
$public = Join-Path (Join-Path (Join-Path (Join-Path (Join-Path $root "target") "dx") "nerdies_weather") "release") "web"
$public = Join-Path $public "public"
$docs = Join-Path $root "docs"

if (Test-Path $public) {
    Remove-Item -LiteralPath $public -Recurse -Force
}

dx build --web --release --base-path $BasePath
& (Join-Path $root "sync-pwa-assets.ps1") -Release

if (Test-Path $docs) {
    Remove-Item -LiteralPath $docs -Recurse -Force
}

New-Item -ItemType Directory -Path $docs | Out-Null
Copy-Item -Path (Join-Path $public "*") -Destination $docs -Recurse -Force

Write-Host "GitHub-Pages-Dateien wurden nach $docs kopiert."
