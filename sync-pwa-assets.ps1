param(
    [switch]$Release
)

$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $MyInvocation.MyCommand.Path
$profile = if ($Release) { "release" } else { "debug" }
$public = Join-Path (Join-Path (Join-Path (Join-Path (Join-Path $root "target") "dx") "nerdies_weather") $profile) "web"
$public = Join-Path $public "public"
$pwa = Join-Path $root "pwa"
$assets = Join-Path $root "assets"

if (-not (Test-Path $public)) {
    throw "Dioxus-Web-Output nicht gefunden: $public. Bitte zuerst den passenden dx build ausfuehren."
}

Copy-Item -LiteralPath (Join-Path $pwa "manifest.webmanifest") -Destination (Join-Path $public "manifest.webmanifest") -Force
Copy-Item -LiteralPath (Join-Path $pwa "sw.js") -Destination (Join-Path $public "sw.js") -Force
Copy-Item -LiteralPath (Join-Path $pwa "register-sw.js") -Destination (Join-Path $public "register-sw.js") -Force
Copy-Item -LiteralPath (Join-Path $assets "favicon.ico") -Destination (Join-Path $public "favicon.ico") -Force
Copy-Item -LiteralPath (Join-Path $assets "pwa-icon-192.png") -Destination (Join-Path $public "pwa-icon-192.png") -Force
Copy-Item -LiteralPath (Join-Path $assets "pwa-icon-512.png") -Destination (Join-Path $public "pwa-icon-512.png") -Force

Write-Host "PWA-Dateien wurden nach $public kopiert."
