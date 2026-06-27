use anyhow::{Context, Result};
use dioxus::prelude::*;
use futures::future::join_all;
use gloo_timers::future::TimeoutFuture;
use serde::Deserialize;
use std::collections::{BTreeMap, HashSet};

const API: &str = "https://api.brightsky.dev";
const TZ: &str = "Europe/Berlin";
const RADAR_SIZE: usize = 28;
const RADAR_MAP_HTML: &str = include_str!("../assets/radar-map.html");
const PV_BASE_FEED_IN_CT_PER_KWH: f64 = 7.78;
const PV_PERFORMANCE_RATIO: f64 = 0.85;
const PV_EXAMPLE_KWP: f64 = 10.0;

fn main() {
    dioxus::launch(App);
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Place {
    city: &'static str,
    district: &'static str,
    state: &'static str,
    lat: f64,
    lon: f64,
    overview: bool,
}

const PLACES: &[Place] = &[
    Place {
        city: "Aachen",
        district: "Mitte",
        state: "NW",
        lat: 50.7753,
        lon: 6.0839,
        overview: true,
    },
    Place {
        city: "Alsdorf",
        district: "Mitte",
        state: "NW",
        lat: 50.8767,
        lon: 6.1640,
        overview: true,
    },
    Place {
        city: "Berlin",
        district: "Charlottenburg-Wilmersdorf",
        state: "BE",
        lat: 52.5079,
        lon: 13.2637,
        overview: false,
    },
    Place {
        city: "Berlin",
        district: "Friedrichshain-Kreuzberg",
        state: "BE",
        lat: 52.5009,
        lon: 13.4201,
        overview: false,
    },
    Place {
        city: "Berlin",
        district: "Mitte",
        state: "BE",
        lat: 52.5200,
        lon: 13.4050,
        overview: true,
    },
    Place {
        city: "Bonn",
        district: "Mitte",
        state: "NW",
        lat: 50.7374,
        lon: 7.0982,
        overview: true,
    },
    Place {
        city: "Bremen",
        district: "Mitte",
        state: "HB",
        lat: 53.0793,
        lon: 8.8017,
        overview: true,
    },
    Place {
        city: "Brocken",
        district: "Harz",
        state: "ST",
        lat: 51.7990,
        lon: 10.6170,
        overview: true,
    },
    Place {
        city: "Celle",
        district: "Mitte",
        state: "NI",
        lat: 52.6226,
        lon: 10.0805,
        overview: true,
    },
    Place {
        city: "Dortmund",
        district: "Innenstadt-West",
        state: "NW",
        lat: 51.5136,
        lon: 7.4653,
        overview: true,
    },
    Place {
        city: "Dresden",
        district: "Altstadt",
        state: "SN",
        lat: 51.0504,
        lon: 13.7373,
        overview: true,
    },
    Place {
        city: "Dresden",
        district: "Neustadt",
        state: "SN",
        lat: 51.0666,
        lon: 13.7553,
        overview: false,
    },
    Place {
        city: "Düsseldorf",
        district: "Bilk",
        state: "NW",
        lat: 51.2052,
        lon: 6.7735,
        overview: false,
    },
    Place {
        city: "Düsseldorf",
        district: "Stadtmitte",
        state: "NW",
        lat: 51.2277,
        lon: 6.7735,
        overview: true,
    },
    Place {
        city: "Erzgebirge",
        district: "Fichtelberg",
        state: "SN",
        lat: 50.4286,
        lon: 12.9542,
        overview: true,
    },
    Place {
        city: "Essen",
        district: "Stadtkern",
        state: "NW",
        lat: 51.4556,
        lon: 7.0116,
        overview: true,
    },
    Place {
        city: "Frankfurt am Main",
        district: "Bockenheim",
        state: "HE",
        lat: 50.1211,
        lon: 8.6387,
        overview: false,
    },
    Place {
        city: "Frankfurt am Main",
        district: "Innenstadt",
        state: "HE",
        lat: 50.1109,
        lon: 8.6821,
        overview: true,
    },
    Place {
        city: "Frankfurt am Main",
        district: "Sachsenhausen",
        state: "HE",
        lat: 50.0994,
        lon: 8.6801,
        overview: false,
    },
    Place {
        city: "Hamburg",
        district: "Altona",
        state: "HH",
        lat: 53.5503,
        lon: 9.9355,
        overview: false,
    },
    Place {
        city: "Hamburg",
        district: "Mitte",
        state: "HH",
        lat: 53.5511,
        lon: 9.9937,
        overview: true,
    },
    Place {
        city: "Hamburg",
        district: "Wandsbek",
        state: "HH",
        lat: 53.5753,
        lon: 10.0752,
        overview: false,
    },
    Place {
        city: "Hannover",
        district: "Mitte",
        state: "NI",
        lat: 52.3759,
        lon: 9.7320,
        overview: true,
    },
    Place {
        city: "Köln",
        district: "Ehrenfeld",
        state: "NW",
        lat: 50.9515,
        lon: 6.9187,
        overview: false,
    },
    Place {
        city: "Köln",
        district: "Innenstadt",
        state: "NW",
        lat: 50.9375,
        lon: 6.9603,
        overview: true,
    },
    Place {
        city: "Köln",
        district: "Nippes",
        state: "NW",
        lat: 50.9655,
        lon: 6.9539,
        overview: false,
    },
    Place {
        city: "Leipzig",
        district: "Plagwitz",
        state: "SN",
        lat: 51.3290,
        lon: 12.3330,
        overview: false,
    },
    Place {
        city: "Leipzig",
        district: "Zentrum",
        state: "SN",
        lat: 51.3397,
        lon: 12.3731,
        overview: true,
    },
    Place {
        city: "München",
        district: "Altstadt-Lehel",
        state: "BY",
        lat: 48.1372,
        lon: 11.5755,
        overview: true,
    },
    Place {
        city: "München",
        district: "Schwabing",
        state: "BY",
        lat: 48.1683,
        lon: 11.5879,
        overview: false,
    },
    Place {
        city: "München",
        district: "Sendling",
        state: "BY",
        lat: 48.1180,
        lon: 11.5392,
        overview: false,
    },
    Place {
        city: "Nürnberg",
        district: "Mitte",
        state: "BY",
        lat: 49.4521,
        lon: 11.0767,
        overview: true,
    },
    Place {
        city: "Paderborn",
        district: "Mitte",
        state: "NW",
        lat: 51.7189,
        lon: 8.7575,
        overview: true,
    },
    Place {
        city: "Serooskerke",
        district: "Walcheren",
        state: "NL",
        lat: 51.5483,
        lon: 3.5954,
        overview: true,
    },
    Place {
        city: "Stuttgart",
        district: "Bad Cannstatt",
        state: "BW",
        lat: 48.8054,
        lon: 9.2140,
        overview: false,
    },
    Place {
        city: "Stuttgart",
        district: "Mitte",
        state: "BW",
        lat: 48.7758,
        lon: 9.1829,
        overview: true,
    },
    Place {
        city: "Zugspitze",
        district: "Gipfel",
        state: "BY",
        lat: 47.4210,
        lon: 10.9853,
        overview: true,
    },
];

#[derive(Clone, Deserialize, Debug, Default)]
struct Source {
    station_name: Option<String>,
    distance: Option<f64>,
}

#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
struct CurrentWeather {
    timestamp: String,
    cloud_cover: Option<f64>,
    condition: Option<String>,
    dew_point: Option<f64>,
    icon: Option<String>,
    precipitation_10: Option<f64>,
    precipitation_30: Option<f64>,
    precipitation_60: Option<f64>,
    pressure_msl: Option<f64>,
    relative_humidity: Option<i64>,
    solar_10: Option<f64>,
    solar_30: Option<f64>,
    solar_60: Option<f64>,
    sunshine_30: Option<f64>,
    sunshine_60: Option<f64>,
    temperature: Option<f64>,
    visibility: Option<i64>,
    wind_direction_10: Option<i64>,
    wind_direction_30: Option<i64>,
    wind_direction_60: Option<i64>,
    wind_speed_10: Option<f64>,
    wind_speed_30: Option<f64>,
    wind_speed_60: Option<f64>,
    wind_gust_speed_10: Option<f64>,
    wind_gust_speed_30: Option<f64>,
    wind_gust_speed_60: Option<f64>,
}

#[derive(Clone, Deserialize, Debug, Default)]
struct CurrentResponse {
    weather: CurrentWeather,
    sources: Vec<Source>,
}

#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
struct WeatherRecord {
    timestamp: String,
    condition: Option<String>,
    icon: Option<String>,
    precipitation: Option<f64>,
    precipitation_probability: Option<i64>,
    solar: Option<f64>,
    sunshine: Option<f64>,
    temperature: Option<f64>,
    wind_speed: Option<f64>,
    wind_gust_speed: Option<f64>,
}

#[derive(Clone, Deserialize, Debug, Default)]
struct WeatherResponse {
    weather: Vec<WeatherRecord>,
}

#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
struct AlertLocation {
    name: Option<String>,
    district: Option<String>,
    state: Option<String>,
}

#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
struct Alert {
    onset: String,
    expires: Option<String>,
    severity: Option<String>,
    event_de: Option<String>,
    headline_de: String,
    description_de: String,
    instruction_de: Option<String>,
}

#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
struct AlertsResponse {
    alerts: Vec<Alert>,
    location: Option<AlertLocation>,
}

#[derive(Clone, Deserialize, Debug)]
struct RadarRecord {
    timestamp: String,
    precipitation_5: Vec<Vec<i64>>,
}

#[derive(Clone, Deserialize, Debug, Default)]
struct RadarResponse {
    radar: Vec<RadarRecord>,
}

#[derive(Clone, Debug)]
struct OverviewCard {
    place: Place,
    current: Option<CurrentWeather>,
    error: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
struct DailySummary {
    date: String,
    min_temp: Option<f64>,
    max_temp: Option<f64>,
    rain_sum: f64,
    solar_sum: f64,
    solar_count: usize,
    sunshine_sum: f64,
    sunshine_count: usize,
    max_pop: Option<i64>,
    max_gust: Option<f64>,
    dominant_condition: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
struct RadarSummary {
    timestamp: String,
    cells: Vec<RadarCell>,
    max_value: i64,
    average_mm: f64,
}

#[derive(Clone, Debug, PartialEq)]
struct RadarCell {
    value: i64,
}

#[derive(Clone, Debug)]
struct DetailData {
    current: CurrentResponse,
    forecast: Vec<DailySummary>,
    hourly: Vec<WeatherRecord>,
    alerts: AlertsResponse,
    national_alerts: AlertsResponse,
    radar: Option<RadarSummary>,
}

#[component]
fn App() -> Element {
    let mut selected = use_signal(|| 0usize);
    let mut live_time = use_signal(current_time_label);

    use_future(move || async move {
        loop {
            live_time.set(current_time_label());
            TimeoutFuture::new(1_000).await;
        }
    });

    let overview = use_resource(|| async move { load_overview().await });
    let detail = use_resource(move || async move {
        let idx = selected();
        load_detail(PLACES[idx]).await
    });

    let selected_place = PLACES[selected()];
    let live_time_text = live_time();

    rsx! {
        document::Stylesheet {
            href: asset!("/assets/main.css")
        }
        document::Link {
            rel: "icon",
            r#type: "image/x-icon",
            href: asset!("/assets/favicon.ico")
        }
        document::Link {
            rel: "shortcut icon",
            r#type: "image/x-icon",
            href: asset!("/assets/favicon.ico")
        }
        main { class: "app-shell",
            section { class: "hero",
                div { class: "hero-inner",
                    h1 { "Live Wetter" }
                    p {
                        "Live Wetter, Stadtteile, 5-Tage-Ausblick, Photovoltaik, Radar und Warnungen für ausgewählte Orte."
                    }
                    p { class: "byline", "Von Frank Noack, Hamburg. In Zusammenarbeit mit den Wetterdaten von Bright Sky" }
                    div { class: "hero-actions",
                        span { class: "pill live-clock", "Live Uhrzeit {live_time_text}" }
                        span { class: "pill", "Zeitzone: Europe/Berlin" }
                    }
                }
            }

            div { class: "content",
                aside { class: "sidebar",
                    div { class: "city-list",
                        for (idx, place) in PLACES.iter().enumerate() {
                            a {
                                href: "#detail-panel",
                                class: if idx == selected() { "place-button active" } else { "place-button" },
                                title: "{place.city} - {place.district}",
                                onclick: move |_| selected.set(idx),
                                div {
                                    strong { "{place.city}" }
                                    span { "{place.district} · {place.state}" }
                                }
                                span { class: "dot" }
                            }
                        }
                    }
                }

                div { class: "main-grid",
                    OverviewPanel { overview }
                    DetailPanel { place: selected_place, detail }
                }
            }

            footer { class: "site-footer",
                "Vielen Dank für die Wetterdaten von Bright Sky"
            }
        }
    }
}

#[component]
fn OverviewPanel(overview: Resource<Result<Vec<OverviewCard>, String>>) -> Element {
    rsx! {
        section { class: "panel",
            div { class: "panel-head",
                div {
                    h2 { "Live Wetter Deutschland" }
                    p { class: "muted", "Stadtzentren als schnelle Deutschland-Übersicht." }
                }
            }
            match overview.read().as_ref() {
                Some(Ok(cards)) => rsx! {
                    div { class: "overview-grid",
                        for card in cards {
                            article { class: "weather-card",
                                h3 { "{card.place.city}" }
                                match &card.current {
                                    Some(weather) => rsx! {
                                        div { class: "temp", "{fmt_temp(weather.temperature)}" }
                                        div { class: "muted", "{condition_label(weather.condition.as_deref(), weather.icon.as_deref())}" }
                                        div { class: "status-row",
                                            span { class: "tag", "Regen 60m {fmt_mm(weather.precipitation_60)}" }
                                            span { class: "tag", "Wind {fmt_speed(weather.wind_speed_60)}" }
                                        }
                                    },
                                    None => rsx! { div { class: "error", "{card.error.as_deref().unwrap_or(\"Keine Daten\")}" } },
                                }
                            }
                        }
                    }
                },
                Some(Err(err)) => rsx! { div { class: "error", "{err}" } },
                None => rsx! { div { class: "loader", "Deutschlanddaten werden geladen..." } },
            }
        }
    }
}

#[component]
fn DetailPanel(place: Place, detail: Resource<Result<DetailData, String>>) -> Element {
    rsx! {
        match detail.read().as_ref() {
            Some(Ok(data)) => rsx! {
                section { id: "detail-panel", class: "panel detail-panel",
                    div { class: "panel-head",
                        div {
                            h2 { "{place.city} · {place.district}" }
                            p { class: "muted", "{source_line(&data.current.sources)} · {short_time(&data.current.weather.timestamp)}" }
                        }
                        span { class: "pill", "{fmt_temp(data.current.weather.temperature)}" }
                    }
                    CurrentMetrics { current: data.current.weather.clone(), forecast: data.forecast.clone() }
                }

                div { class: "split",
                    ForecastPanel { forecast: data.forecast.clone(), hourly: data.hourly.clone() }
                    RadarPanel { place, radar: data.radar.clone() }
                }

                AlertsPanel { alerts: data.alerts.clone(), national_alerts: data.national_alerts.clone() }
            },
            Some(Err(err)) => rsx! { section { class: "panel", div { class: "error", "{err}" } } },
            None => rsx! { section { class: "panel loader", "Details werden geladen..." } },
        }
    }
}

#[component]
fn CurrentMetrics(current: CurrentWeather, forecast: Vec<DailySummary>) -> Element {
    let weather_metrics = vec![
        (
            "Zustand",
            condition_label(current.condition.as_deref(), current.icon.as_deref()),
            "Aktuelle Wetterlage, ins Deutsche übersetzt.",
        ),
        ("Temperatur", fmt_temp(current.temperature), "Gemessene Lufttemperatur in 2 m Höhe."),
        ("Taupunkt", fmt_temp(current.dew_point), "Temperatur, ab der die Luft Feuchtigkeit abgibt."),
        ("Luftfeuchte", fmt_percent_i(current.relative_humidity), "Relative Luftfeuchte in Prozent."),
        ("Luftdruck", fmt_hpa(current.pressure_msl), "Auf Meereshöhe umgerechneter Luftdruck."),
        ("Sichtweite", fmt_m(current.visibility), "Geschätzte meteorologische Sichtweite."),
        ("Bewölkung", fmt_percent_f(current.cloud_cover), "Anteil des bedeckten Himmels."),
        ("Regen 10 min", fmt_mm(current.precipitation_10), "Niederschlag der letzten 10 Minuten."),
        ("Regen 30 min", fmt_mm(current.precipitation_30), "Niederschlag der letzten 30 Minuten."),
        ("Regen 60 min", fmt_mm(current.precipitation_60), "Niederschlag der letzten 60 Minuten."),
        ("Wind 10 min", fmt_speed(current.wind_speed_10), "Mittlere Windgeschwindigkeit der letzten 10 Minuten."),
        ("Wind 30 min", fmt_speed(current.wind_speed_30), "Mittlere Windgeschwindigkeit der letzten 30 Minuten."),
        ("Wind 60 min", fmt_speed(current.wind_speed_60), "Mittlere Windgeschwindigkeit der letzten 60 Minuten."),
        ("Böe 10 min", fmt_speed(current.wind_gust_speed_10), "Stärkste Windböe der letzten 10 Minuten."),
        ("Böe 30 min", fmt_speed(current.wind_gust_speed_30), "Stärkste Windböe der letzten 30 Minuten."),
        ("Böe 60 min", fmt_speed(current.wind_gust_speed_60), "Stärkste Windböe der letzten 60 Minuten."),
        ("Windrichtung 10", fmt_direction(current.wind_direction_10), "Windrichtung in Grad, gemittelt über 10 Minuten."),
        ("Windrichtung 30", fmt_direction(current.wind_direction_30), "Windrichtung in Grad, gemittelt über 30 Minuten."),
        ("Windrichtung 60", fmt_direction(current.wind_direction_60), "Windrichtung in Grad, gemittelt über 60 Minuten."),
    ];
    let solar_metrics = vec![
        ("Solarstrahlung 10 min", fmt_solar(current.solar_10), "Eingestrahlte Sonnenenergie pro Quadratmeter in den letzten 10 Minuten; kein direkter Photovoltaik-Ertrag."),
        ("Solarstrahlung 30 min", fmt_solar(current.solar_30), "Eingestrahlte Sonnenenergie pro Quadratmeter in den letzten 30 Minuten; Photovoltaik-Ertrag hängt von Anlage und Ausrichtung ab."),
        ("Solarstrahlung 60 min", fmt_solar(current.solar_60), "Eingestrahlte Sonnenenergie pro Quadratmeter in den letzten 60 Minuten in kWh/m²."),
    ];

    rsx! {
        div { class: "metric-section-title metric-section-title-first",
            h3 { "Aktuelle Wetterwerte" }
        }
        div { class: "metric-grid",
            for (label, value, help) in weather_metrics {
                div { class: "metric",
                    span { "{label}" }
                    strong { "{value}" }
                    small { "{help}" }
                }
            }
        }
        div { class: "metric-section-title",
            h3 { "Solarstrahlung" }
        }
        div { class: "metric-grid metric-grid-solar",
            for (label, value, help) in solar_metrics {
                div { class: "metric",
                    span { "{label}" }
                    strong { "{value}" }
                    small { "{help}" }
                }
            }
        }
        PvDailyPanel { forecast }
        div { class: "solar-info",
            strong { "Einordnung für Photovoltaik" }
            p { "Die Solarstrahlung ist die gemessene Energie pro Quadratmeter. Ein Balkonkraftwerk mit etwa 0,8 kWp erzeugt daraus ungefähr 8% des angezeigten 10-kWp-Tageswerts; eine typische Dachanlage mit 10 kWp entspricht dem roten Tageswert im Photovoltaik-Block. Der tatsächliche Ertrag hängt von Ausrichtung, Neigung, Verschattung und Wechselrichter ab." }
        }
    }
}

#[component]
fn PvDailyPanel(forecast: Vec<DailySummary>) -> Element {
    let pv_tariff = active_pv_tariff();
    let pv_period = pv_tariff.period_label();
    let pv_note = format!(
        "Berechnet für eine Beispielanlage mit {:.0} kWp am gewählten Ort, automatisch berechnetem EEG-Einspeisewert {} und {} Anlagenfaktor. Sonnenstunden sind Wetterdaten, nicht die Anlagenleistung. Tarifperiode: {pv_period}.",
        PV_EXAMPLE_KWP,
        fmt_ct_per_kwh(pv_tariff.ct_per_kwh),
        fmt_percent_decimal(PV_PERFORMANCE_RATIO)
    );

    rsx! {
        div { class: "pv-panel",
            div { class: "pv-panel-head",
                h3 { "Photovoltaik Tagesertrag" }
                p { class: "muted", "{pv_note}" }
            }
            div { class: "pv-day-grid",
                for day in forecast {
                    div { class: "pv-day-card",
                        span { "{format_date(&day.date)}" }
                        strong { class: "pv-revenue", "{fmt_pv_revenue_for_kwp(&day, pv_tariff, PV_EXAMPLE_KWP)}" }
                        small { "{fmt_pv_revenue(&day, pv_tariff)}" }
                        small { "{fmt_pv_yield(&day)}" }
                        small { "{fmt_sun_hours_day(&day)}" }
                    }
                }
            }
        }
    }
}

#[component]
fn ForecastPanel(forecast: Vec<DailySummary>, hourly: Vec<WeatherRecord>) -> Element {
    let pv_tariff = active_pv_tariff();
    let pv_tariff_line = format!(
        "{} Stundenwerte mit Photovoltaik-Schätzung pro 1 kWp, automatisch berechnetem EEG-Einspeisewert {} und {} Anlagenfaktor.",
        hourly.len(),
        fmt_ct_per_kwh(pv_tariff.ct_per_kwh),
        fmt_percent_decimal(PV_PERFORMANCE_RATIO)
    );
    let pv_period = pv_tariff.period_label();

    rsx! {
        section { class: "panel",
            div { class: "panel-head",
                div {
                    h2 { "5-Tage-Ausblick" }
                    p { class: "muted", "{pv_tariff_line} Tarifperiode: {pv_period}." }
                }
            }
            div { class: "forecast-days",
                for day in forecast {
                    div { class: "day-card",
                        strong { "{format_date(&day.date)}" }
                        div { "{fmt_temp(day.min_temp)} bis {fmt_temp(day.max_temp)}" }
                        small { "Temperaturspanne: tiefster bis höchster Wert des Tages." }
                        div { class: "muted", "{condition_label(day.dominant_condition.as_deref(), None)}" }
                        div { class: "bar", span { style: "width: {rain_width(day.rain_sum)}%;" } }
                        div { class: "status-row",
                            span { class: "tag", "Regen {fmt_rain_sum(day.rain_sum)}" }
                            span { class: "tag", "Regenchance {fmt_percent_i(day.max_pop)}" }
                            span { class: "tag", "Böe {fmt_speed(day.max_gust)}" }
                        }
                        div { class: "status-row pv-row",
                            span { class: "tag", "{fmt_solar_day(&day)}" }
                            span { class: "tag", "{fmt_pv_yield(&day)}" }
                            span { class: "tag", "{fmt_pv_revenue(&day, pv_tariff)}" }
                        }
                        if is_extreme(&day) {
                            span { class: "extreme", "{extreme_label(&day)}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn RadarPanel(
    place: Place,
    radar: Option<RadarSummary>,
) -> Element {
    let radar_src = radar_map_src(place);

    rsx! {
        section { class: "panel radar-panel",
            div { class: "panel-head",
                div {
                    h2 { "Radar" }
                    p { class: "muted", "Niederschlagsradar mit Deutschlandansicht im Vollbild." }
                }
            }
            div { class: "radar-layout",
                match radar {
                    Some(_) => rsx! {
                        div { class: "radar-map-frame",
                            iframe {
                                class: "radar-frame",
                                src: "{radar_src}",
                                title: "Bright Sky Niederschlagsradar",
                                allow: "fullscreen",
                                allowfullscreen: true
                            }
                        }
                    },
                    None => rsx! { div { class: "loader", "Keine Radardaten für diesen Ausschnitt." } },
                }
            }
        }
    }
}

fn unique_alerts(alerts: &[Alert]) -> Vec<Alert> {
    let mut seen = HashSet::new();
    alerts
        .iter()
        .filter(|alert| alert_starts_today_or_tomorrow(alert))
        .filter(|alert| seen.insert(alert_group_key(alert)))
        .cloned()
        .collect()
}

fn alert_starts_today_or_tomorrow(alert: &Alert) -> bool {
    let Some(onset_date) = alert_date_key(&alert.onset) else {
        return true;
    };
    let today = js_sys::Date::new_0();
    let tomorrow = js_sys::Date::new_0();
    tomorrow.set_date(today.get_date() + 1);
    let today_key = js_date_key(&today);
    let tomorrow_key = js_date_key(&tomorrow);

    onset_date >= today_key && onset_date <= tomorrow_key
}

fn alert_date_key(value: &str) -> Option<i32> {
    let date = value
        .trim()
        .replace('T', " ")
        .split_whitespace()
        .next()?
        .to_string();

    if date.contains('-') {
        let parts = date.split('-').collect::<Vec<_>>();
        if parts.len() == 3 {
            if parts[0].len() == 4 {
                return date_key(parts[0], parts[1], parts[2]);
            }
            return date_key(parts[2], parts[1], parts[0]);
        }
    }

    if date.contains('.') {
        let parts = date.split('.').collect::<Vec<_>>();
        if parts.len() == 3 {
            return date_key(parts[2], parts[1], parts[0]);
        }
    }

    None
}

fn date_key(year: &str, month: &str, day: &str) -> Option<i32> {
    Some(year.parse::<i32>().ok()? * 10_000 + month.parse::<i32>().ok()? * 100 + day.parse::<i32>().ok()?)
}

fn js_date_key(date: &js_sys::Date) -> i32 {
    date.get_full_year() as i32 * 10_000 + (date.get_month() as i32 + 1) * 100 + date.get_date() as i32
}

fn alert_group_key(alert: &Alert) -> String {
    let event = alert
        .event_de
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or(&alert.headline_de);
    format!(
        "{}|{}",
        normalize_alert_key(event),
        alert
            .severity
            .as_deref()
            .map(normalize_alert_key)
            .unwrap_or_default()
    )
}

fn normalize_alert_key(value: &str) -> String {
    value
        .trim()
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn current_time_label() -> String {
    let now = js_sys::Date::new_0();
    format!(
        "{:02}:{:02}:{:02}",
        now.get_hours(),
        now.get_minutes(),
        now.get_seconds()
    )
}

#[component]
fn AlertsPanel(alerts: AlertsResponse, national_alerts: AlertsResponse) -> Element {
    let local_title = alerts
        .location
        .as_ref()
        .and_then(|loc| loc.name.clone())
        .unwrap_or_else(|| "gewählter Ort".to_string());
    let local_alerts = unique_alerts(&alerts.alerts);
    let local_alert_groups = local_alerts.iter().map(alert_group_key).collect::<HashSet<_>>();
    let mut national = unique_alerts(&national_alerts.alerts)
        .into_iter()
        .filter(|alert| !local_alert_groups.contains(&alert_group_key(alert)))
        .collect::<Vec<_>>();
    national.sort_by_key(|alert| severity_rank(alert.severity.as_deref()));
    national.reverse();
    national.truncate(8);

    rsx! {
        section { class: "panel",
            div { class: "panel-head",
                div {
                    h2 { "DWD-Warnungen" }
                    p { class: "muted", "Lokal für {local_title}; darunter die wichtigsten deutschlandweiten Warnungen." }
                }
            }
            div { class: "alerts",
                h3 { "Lokal für {local_title}" }
                if local_alerts.is_empty() {
                    div { class: "alert-card", strong { "Keine lokale Warnung" } span { class: "muted", "Für diesen Ort liegt aktuell keine Bright-Sky-Warnung vor." } }
                }
                for alert in local_alerts.iter().take(6) {
                    AlertView { alert: alert.clone() }
                }
                h3 { "Deutschlandweit" }
                if national.is_empty() {
                    div { class: "alert-card", strong { "Keine deutschlandweiten Warnungen" } }
                }
                for alert in national {
                    AlertView { alert }
                }
            }
        }
    }
}

#[component]
fn AlertView(alert: Alert) -> Element {
    let severity = alert.severity.clone().unwrap_or_else(|| "info".to_string());
    rsx! {
        article { class: "alert-card {severity}",
            strong { "{alert.headline_de}" }
            div { class: "muted", "{alert.description_de.trim()}" }
            if let Some(instruction) = alert.instruction_de.as_ref().filter(|value| !value.trim().is_empty()) {
                p { "{instruction.trim()}" }
            }
            div { class: "alert-meta",
                span { class: "tag", "{severity_label(&severity)}" }
                if let Some(event) = alert.event_de.as_ref() {
                    span { class: "tag", "{event}" }
                }
                span { class: "tag", "ab {short_time(&alert.onset)}" }
                if let Some(expires) = alert.expires.as_ref() {
                    span { class: "tag", "bis {short_time(expires)}" }
                }
            }
        }
    }
}

async fn load_overview() -> Result<Vec<OverviewCard>, String> {
    let places = PLACES
        .iter()
        .copied()
        .filter(|place| place.overview)
        .collect::<Vec<_>>();
    let results = join_all(places.iter().map(|place| fetch_current(*place))).await;
    Ok(places
        .into_iter()
        .zip(results)
        .map(|(place, result)| match result {
            Ok(response) => OverviewCard {
                place,
                current: Some(response.weather),
                error: None,
            },
            Err(err) => OverviewCard {
                place,
                current: None,
                error: Some(format!("{err:#}")),
            },
        })
        .collect())
}

async fn load_detail(place: Place) -> Result<DetailData, String> {
    let current = fetch_current(place);
    let forecast = fetch_weather(place);
    let alerts = fetch_alerts(Some(place));
    let national_alerts = fetch_alerts(None);
    let radar = fetch_radar(place);
    let (current, forecast, alerts, national_alerts, radar) =
        futures::join!(current, forecast, alerts, national_alerts, radar);

    let current = current.map_err(|err| format!("{err:#}"))?;
    let weather = forecast.map_err(|err| format!("{err:#}"))?;
    let alerts = alerts.unwrap_or_default();
    let national_alerts = national_alerts.unwrap_or_default();
    let radar = radar.ok().flatten();

    Ok(DetailData {
        current,
        forecast: summarize_days(&weather.weather),
        hourly: weather.weather,
        alerts,
        national_alerts,
        radar,
    })
}

async fn fetch_current(place: Place) -> Result<CurrentResponse> {
    let url = format!(
        "{API}/current_weather?lat={}&lon={}&max_dist=500000&tz={}&units=dwd",
        place.lat,
        place.lon,
        encode(TZ)
    );
    get_json(&url).await
}

async fn fetch_weather(place: Place) -> Result<WeatherResponse> {
    let (date, last_date) = five_day_range();
    let url = format!(
        "{API}/weather?lat={}&lon={}&date={}&last_date={}&tz={}&units=dwd",
        place.lat,
        place.lon,
        encode(&date),
        encode(&last_date),
        encode(TZ)
    );
    get_json(&url).await
}

async fn fetch_alerts(place: Option<Place>) -> Result<AlertsResponse> {
    let url = match place {
        Some(place) => format!(
            "{API}/alerts?lat={}&lon={}&tz={}",
            place.lat,
            place.lon,
            encode(TZ)
        ),
        None => format!("{API}/alerts?tz={}", encode(TZ)),
    };
    get_json(&url).await
}

async fn fetch_radar(place: Place) -> Result<Option<RadarSummary>> {
    let url = format!(
        "{API}/radar?lat={}&lon={}&distance=30000&format=plain&tz={}",
        place.lat,
        place.lon,
        encode(TZ)
    );
    let response: RadarResponse = get_json(&url).await?;
    Ok(response.radar.last().map(summarize_radar))
}

async fn get_json<T: for<'de> Deserialize<'de>>(url: &str) -> Result<T> {
    reqwest::get(url)
        .await
        .with_context(|| format!("Request fehlgeschlagen: {url}"))?
        .error_for_status()
        .with_context(|| format!("Bright Sky meldet einen Fehler: {url}"))?
        .json::<T>()
        .await
        .context("Bright-Sky-Antwort konnte nicht gelesen werden")
}

fn summarize_days(records: &[WeatherRecord]) -> Vec<DailySummary> {
    let mut days = BTreeMap::<String, DailySummary>::new();

    for record in records {
        let date = record.timestamp.chars().take(10).collect::<String>();
        let entry = days.entry(date.clone()).or_insert_with(|| DailySummary {
            date,
            ..DailySummary::default()
        });

        if let Some(temp) = record.temperature {
            entry.min_temp = Some(entry.min_temp.map_or(temp, |current| current.min(temp)));
            entry.max_temp = Some(entry.max_temp.map_or(temp, |current| current.max(temp)));
        }
        entry.rain_sum += record.precipitation.unwrap_or(0.0);
        if let Some(solar) = record.solar {
            entry.solar_sum += solar;
            entry.solar_count += 1;
        }
        if let Some(sunshine) = record.sunshine {
            entry.sunshine_sum += sunshine;
            entry.sunshine_count += 1;
        }
        if let Some(pop) = record.precipitation_probability {
            entry.max_pop = Some(entry.max_pop.map_or(pop, |current| current.max(pop)));
        }
        if let Some(gust) = record.wind_gust_speed.or(record.wind_speed) {
            entry.max_gust = Some(entry.max_gust.map_or(gust, |current| current.max(gust)));
        }
        if entry.dominant_condition.is_none() {
            entry.dominant_condition = record.condition.clone().or_else(|| record.icon.clone());
        }
    }

    days.into_values().take(5).collect()
}

fn summarize_radar(record: &RadarRecord) -> RadarSummary {
    let rows = record.precipitation_5.len();
    let cols = record.precipitation_5.first().map_or(0, Vec::len);
    let y_step = (rows.max(RADAR_SIZE) as f64) / RADAR_SIZE as f64;
    let x_step = (cols.max(RADAR_SIZE) as f64) / RADAR_SIZE as f64;
    let mut cells = Vec::with_capacity(RADAR_SIZE * RADAR_SIZE);
    let mut max_value = 0;
    let mut total = 0i64;
    let mut count = 0i64;

    for y in 0..RADAR_SIZE {
        for x in 0..RADAR_SIZE {
            let start_y = (y as f64 * y_step).floor() as usize;
            let end_y = (((y + 1) as f64 * y_step).ceil() as usize)
                .min(rows)
                .max(start_y + 1);
            let start_x = (x as f64 * x_step).floor() as usize;
            let end_x = (((x + 1) as f64 * x_step).ceil() as usize)
                .min(cols)
                .max(start_x + 1);
            let mut local_max = 0;
            for row in record.precipitation_5.iter().take(end_y).skip(start_y) {
                for value in row.iter().take(end_x).skip(start_x) {
                    local_max = local_max.max(*value);
                    max_value = max_value.max(*value);
                    total += *value;
                    count += 1;
                }
            }
            cells.push(RadarCell { value: local_max });
        }
    }

    RadarSummary {
        timestamp: record.timestamp.clone(),
        cells,
        max_value,
        average_mm: if count == 0 {
            0.0
        } else {
            (total as f64 / count as f64) / 100.0
        },
    }
}

fn five_day_range() -> (String, String) {
    let start = js_sys::Date::new_0();
    let end = js_sys::Date::new_0();
    end.set_date(start.get_date() + 5);
    (date_string(&start), date_string(&end))
}

fn date_string(date: &js_sys::Date) -> String {
    format!(
        "{:04}-{:02}-{:02}",
        date.get_full_year(),
        date.get_month() + 1,
        date.get_date()
    )
}

fn encode(value: &str) -> String {
    urlencoding::encode(value).into_owned()
}

fn radar_map_src(place: Place) -> String {
    let label = format!("{} · {}", place.city, place.district);
    let html = RADAR_MAP_HTML
        .replace("__RADAR_LAT__", &place.lat.to_string())
        .replace("__RADAR_LON__", &place.lon.to_string())
        .replace("__RADAR_NAME__", &js_string(&label));
    format!("data:text/html;charset=utf-8,{}", encode(&html))
}

fn js_string(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

fn fmt_temp(value: Option<f64>) -> String {
    value.map_or_else(|| "keine Daten".to_string(), |v| format!("{v:.1} °C"))
}

fn fmt_mm(value: Option<f64>) -> String {
    value.map_or_else(|| "keine Daten".to_string(), |v| format!("{v:.1} mm"))
}

fn fmt_speed(value: Option<f64>) -> String {
    value.map_or_else(|| "keine Daten".to_string(), |v| format!("{v:.0} km/h"))
}

fn fmt_hpa(value: Option<f64>) -> String {
    value.map_or_else(|| "keine Daten".to_string(), |v| format!("{v:.0} hPa"))
}

fn fmt_m(value: Option<i64>) -> String {
    value.map_or_else(
        || "keine Daten".to_string(),
        |v| {
            if v >= 1000 {
                format!("{:.1} km", v as f64 / 1000.0)
            } else {
                format!("{v} m")
            }
        },
    )
}

fn fmt_percent_i(value: Option<i64>) -> String {
    value.map_or_else(|| "keine Daten".to_string(), |v| format!("{v}%"))
}

fn fmt_percent_f(value: Option<f64>) -> String {
    value.map_or_else(|| "keine Daten".to_string(), |v| format!("{v:.0}%"))
}

fn fmt_direction(value: Option<i64>) -> String {
    value.map_or_else(|| "keine Daten".to_string(), |v| format!("{v}°"))
}

fn fmt_solar(value: Option<f64>) -> String {
    value.map_or_else(|| "keine Messung".to_string(), |v| format!("{v:.3} kWh/m²"))
}

fn fmt_ct_per_kwh(value: f64) -> String {
    format!("{value:.2} ct/kWh").replace('.', ",")
}

#[derive(Clone, Copy)]
struct PvTariff {
    ct_per_kwh: f64,
    start_year: i32,
    start_month: i32,
    end_year: i32,
    end_month: i32,
    end_day: i32,
}

impl PvTariff {
    fn period_label(&self) -> String {
        format!(
            "{} bis {}",
            format_period_date(self.start_year, self.start_month, 1),
            format_period_date(self.end_year, self.end_month, self.end_day)
        )
    }
}

fn active_pv_tariff() -> PvTariff {
    let today = js_sys::Date::new_0();
    let year = today.get_full_year() as i32;
    let month = today.get_month() as i32 + 1;
    let (start_year, start_month, end_year, end_month, end_day) = if year < 2026 || (year == 2026 && month < 2) {
        (2026, 2, 2026, 7, 31)
    } else if month == 1 {
        (year - 1, 8, year, 1, 31)
    } else if month <= 7 {
        (year, 2, year, 7, 31)
    } else {
        (year, 8, year + 1, 1, 31)
    };
    let period_index = ((start_year - 2026) * 2 + if start_month == 8 { 1 } else { 0 }).max(0);

    PvTariff {
        ct_per_kwh: PV_BASE_FEED_IN_CT_PER_KWH * 0.99_f64.powi(period_index),
        start_year,
        start_month,
        end_year,
        end_month,
        end_day,
    }
}

fn format_period_date(year: i32, month: i32, day: i32) -> String {
    format!("{day:02}.{month:02}.{year:04}")
}

fn fmt_percent_decimal(value: f64) -> String {
    format!("{:.0}%", value * 100.0)
}

fn pv_yield(day: &DailySummary) -> Option<f64> {
    (day.solar_count > 0).then_some(day.solar_sum * PV_PERFORMANCE_RATIO)
}

fn fmt_solar_day(day: &DailySummary) -> String {
    if day.solar_count == 0 {
        "Solar keine Daten".to_string()
    } else {
        format!("Solar {:.1} kWh/m²", day.solar_sum).replace('.', ",")
    }
}

fn fmt_pv_yield(day: &DailySummary) -> String {
    pv_yield(day).map_or_else(
        || "Photovoltaik keine Daten".to_string(),
        |value| format!("Photovoltaik {:.1} kWh/kWp", value).replace('.', ","),
    )
}

fn fmt_pv_revenue(day: &DailySummary, tariff: PvTariff) -> String {
    pv_yield(day).map_or_else(
        || "pro 1 kWp keine Daten".to_string(),
        |value| format!("pro 1 kWp {:.2} €/Tag", value * tariff.ct_per_kwh / 100.0).replace('.', ","),
    )
}

fn fmt_pv_revenue_for_kwp(day: &DailySummary, tariff: PvTariff, kwp: f64) -> String {
    pv_yield(day).map_or_else(
        || format!("Tageswert {:.0} kWp keine Daten", kwp),
        |value| {
            let revenue = format!("{:.2}", value * kwp * tariff.ct_per_kwh / 100.0).replace('.', ",");
            format!("Tageswert {kwp:.0} kWp ca. {revenue} €/Tag")
        },
    )
}

fn fmt_sun_hours_day(day: &DailySummary) -> String {
    if day.sunshine_count == 0 {
        "Sonnenschein keine Daten".to_string()
    } else {
        format!("Sonnenschein {:.1} h", day.sunshine_sum / 60.0).replace('.', ",")
    }
}

fn condition_label(condition: Option<&str>, icon: Option<&str>) -> String {
    match condition.or(icon) {
        Some("dry") | Some("clear-day") | Some("clear-night") => "trocken".to_string(),
        Some("fog") => "Nebel".to_string(),
        Some("rain") => "Regen".to_string(),
        Some("sleet") => "Schneeregen".to_string(),
        Some("snow") => "Schnee".to_string(),
        Some("hail") => "Hagel".to_string(),
        Some("thunderstorm") => "Gewitter".to_string(),
        Some("cloudy") => "bewölkt".to_string(),
        Some("partly-cloudy-day") | Some("partly-cloudy-night") => "teils bewölkt".to_string(),
        Some("wind") => "windig".to_string(),
        Some(value) => value.replace('-', " "),
        None => "keine Daten".to_string(),
    }
}

fn source_line(sources: &[Source]) -> String {
    sources.first().map_or_else(
        || "Nächste Wetterstation: DWD-Station".to_string(),
        |source| {
            let name = source.station_name.as_deref().unwrap_or("DWD-Station");
            let distance = source
                .distance
                .map(|m| format!(" · {} km", fmt_decimal(m / 1000.0, 1)))
                .unwrap_or_default();
            format!("Nächste Wetterstation: {name}{distance}")
        },
    )
}

fn short_time(value: &str) -> String {
    let normalized = value.replace('T', " ");
    let date = normalized.chars().take(10).collect::<String>();
    let time = normalized.chars().skip(11).take(5).collect::<String>();

    if date.len() == 10 && time.len() == 5 {
        format!("{} - {} Uhr", format_date(&date), time)
    } else {
        value.to_string()
    }
}

fn format_date(value: &str) -> String {
    let parts = value.split('-').collect::<Vec<_>>();
    if parts.len() == 3 {
        format!("{}-{}-{}", parts[2], parts[1], parts[0])
    } else {
        value.to_string()
    }
}

fn fmt_decimal(value: f64, digits: usize) -> String {
    format!("{value:.digits$}").replace('.', ",")
}

fn rain_width(rain_sum: f64) -> i64 {
    ((rain_sum / 25.0) * 100.0).round().clamp(4.0, 100.0) as i64
}

fn is_extreme(day: &DailySummary) -> bool {
    day.max_temp.is_some_and(|v| v >= 30.0)
        || day.min_temp.is_some_and(|v| v <= -10.0)
        || day.rain_sum >= 25.0
}

fn extreme_label(day: &DailySummary) -> String {
    if day.max_temp.is_some_and(|v| v >= 35.0) {
        "extreme Hitze".to_string()
    } else if day.max_temp.is_some_and(|v| v >= 30.0) {
        "Hitze".to_string()
    } else if day.min_temp.is_some_and(|v| v <= -10.0) {
        "strenger Frost".to_string()
    } else {
        "Starkregen möglich".to_string()
    }
}

fn fmt_rain_sum(value: f64) -> String {
    format!("{value:.1} mm")
}

fn severity_rank(value: Option<&str>) -> i32 {
    match value {
        Some("extreme") => 4,
        Some("severe") => 3,
        Some("moderate") => 2,
        Some("minor") => 1,
        _ => 0,
    }
}

fn severity_label(value: &str) -> &'static str {
    match value {
        "extreme" => "extrem",
        "severe" => "schwer",
        "moderate" => "moderat",
        "minor" => "gering",
        _ => "Hinweis",
    }
}
