use std::time::Duration;

use anyhow::{bail, Context};
use serde::Deserialize;

use crate::{
    temperature::{Celsius, Temperature},
    Provider, Weather,
};

pub struct WeatherStack {
    base_url: String,
    api_key: String,
}

impl WeatherStack {
    #[must_use]
    pub fn new(api_key: &str) -> Self {
        Self {
            base_url: "https://api.weatherstack.com/current".to_string(),
            api_key: api_key.to_owned(),
        }
    }
}

impl Provider for WeatherStack {
    fn get_weather(&self, location: &str) -> anyhow::Result<Weather> {
        let resp = reqwest::blocking::Client::new()
            .get(&self.base_url)
            .query(&[("query", location), ("access_key", &self.api_key)])
            .timeout(Duration::from_secs(1))
            .send()?;
        resp.error_for_status_ref()?;
        deserialize(&resp.text()?)
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct WSWeather {
    location: WSLocation,
    current: WSCurrent,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct WSLocation {
    name: String,
    country: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct WSCurrent {
    temperature: i16,
    weather_descriptions: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct WSErrorResponse {
    error: WSError,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct WSError {
    info: String,
}

fn deserialize(json: &str) -> anyhow::Result<Weather> {
    if let Ok(resp) = serde_json::from_str::<WSErrorResponse>(json) {
        bail!(resp.error.info)
    }
    let ws: WSWeather = serde_json::from_str(json).with_context(|| json.to_owned())?;
    let Some(summary) = ws.current.weather_descriptions.into_iter().next() else {
        bail!("invalid API response");
    };
    Ok(Weather {
        location: format!("{},{}", ws.location.name, ws.location.country),
        temperature: Temperature::from::<Celsius>(ws.current.temperature),
        summary,
    })
}

#[cfg(test)]
mod tests {
    use http::StatusCode;
    use httpmock::{Method, MockServer};

    use std::fs;

    use super::*;

    #[test]
    fn get_weather_fn_makes_correct_api_call() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(Method::GET)
                .query_param("query", "London,UK")
                .query_param("access_key", "dummy api key");
            then.status(StatusCode::OK.into())
                .header("content-type", "application/json")
                .body_from_file("tests/data/weatherstack.json");
        });
        let mut ws = WeatherStack::new("dummy api key");
        ws.base_url = server.base_url();
        let weather = ws.get_weather("London,UK").unwrap();
        assert_eq!(
            weather,
            Weather {
                location: "London,United Kingdom".into(),
                temperature: Temperature::from::<Celsius>(11.0),
                summary: "Sunny".into(),
            },
            "wrong weather"
        );
    }

    #[test]
    fn deserialize_correctly_parses_test_data() {
        let json = fs::read_to_string("tests/data/weatherstack.json").unwrap();
        assert_eq!(
            deserialize(&json).unwrap(),
            Weather {
                location: "London,United Kingdom".into(),
                temperature: Temperature::from::<Celsius>(11.0),
                summary: "Sunny".into(),
            },
            "wrong weather"
        );
    }
}
