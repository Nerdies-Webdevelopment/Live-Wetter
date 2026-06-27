# Live Wetter

Eine Dioxus-Web-App fuer Live-Wetterdaten, Stadtteile, 5-Tage-Ausblick, Photovoltaik-Ertrag, Radar und DWD-Warnungen fuer ausgewaehlte Orte.

Die App nutzt Wetterdaten von [Bright Sky](https://brightsky.dev/) und laeuft komplett im Browser. Es ist kein eigener API-Schluessel notwendig.

## Funktionen

- Live-Wetterwerte fuer ausgewaehlte deutsche Orte und Stadtteile
- Deutschland-Uebersicht mit Temperatur, Wetterlage, Regen und Wind
- Detailansicht mit Wetterstation, Messwerten und 5-Tage-Ausblick
- Photovoltaik-Tagesertrag mit dynamischem Einspeisewert
- Niederschlagsradar mit Vollbildmodus
- Lokale und deutschlandweite DWD-Warnungen
- Progressive Web App mit Manifest, Service Worker und App-Icons
- Deployment ueber GitHub Pages per GitHub Actions

## Voraussetzungen

- Rust
- `wasm32-unknown-unknown` Target
- Dioxus CLI 0.7.3

```powershell
rustup target add wasm32-unknown-unknown
cargo install dioxus-cli --version 0.7.3 --locked
```

## Lokal starten

```powershell
dx serve --web
```

Die App ist danach standardmaessig unter `http://127.0.0.1:8080/` erreichbar.

## PWA-Build lokal erzeugen

```powershell
powershell -ExecutionPolicy Bypass -File .\build-pwa.ps1
```

Das Skript fuehrt `dx build --web` aus und kopiert danach diese PWA-Dateien in den Dioxus-Web-Output:

- `manifest.webmanifest`
- `sw.js`
- `register-sw.js`
- `favicon.ico`
- `pwa-icon-192.png`
- `pwa-icon-512.png`

Fuer einen Release-Build:

```powershell
powershell -ExecutionPolicy Bypass -File .\build-pwa.ps1 -Release
```

## GitHub Pages

Das Repository enthaelt den Workflow `.github/workflows/pages.yml`.

Die fertige GitHub-Page liegt im Ordner `docs/`. Dadurch kann GitHub Pages ohne langen Rust-/Dioxus-Build direkt veroeffentlichen.

Zum Aktualisieren der GitHub Page:

```powershell
powershell -ExecutionPolicy Bypass -File .\update-pages.ps1
git add .
git commit -m "Update GitHub Pages build"
git push
```

Bei jedem Push auf `main` veroeffentlicht GitHub Actions den Inhalt aus `docs/`.

Die Page ist nach erfolgreichem Workflow typischerweise unter dieser Adresse erreichbar:

```text
https://<github-benutzername>.github.io/<repository-name>/
```

## Datenquellen

- Wetterdaten, Radar und DWD-Warnungen: [Bright Sky](https://brightsky.dev/)
- Kartendaten: OpenStreetMap

## Hinweise

Die PWA kann die App-Huelle und statische Dateien offline bereitstellen. Aktuelle Wetterdaten, Warnungen und Radar brauchen weiterhin eine Internetverbindung.
