#![doc = include_str!("../README.md")]
use anyhow::{Context, Result};
use reqwest::blocking::RequestBuilder;
use serde_json::Value;

use std::time::Duration;

#[derive(Debug, PartialEq)]
/// The weather conditions for a given location.
pub struct Weather {
    pub location: String,
    pub temperature: Temperature,
    pub summary: String,
}

#[derive(Debug, PartialEq)]
/// The temperature for a given location.
pub struct Temperature(f64);

impl Temperature {
    #[must_use]
    /// Creates a Temperature from an `f64` value expressed in degrees Celsius.
    pub fn from_celsius(val: f64) -> Self {
        Self(val)
    }

    #[must_use]
    /// Converts a Temperature to an `f64` value expressed in degrees Celsius.
    pub fn as_celsius(&self) -> f64 {
        self.0
    }

    #[must_use]
    /// Converts a Temperature to an `f64` value expressed in degrees Fahrenheit.
    pub fn as_fahrenheit(&self) -> f64 {
        self.0 * 1.8 + 32.0
    }
}

/// Configuration for the Weatherstack client.
pub struct Weatherstack {
    base_url: String,
    api_key: String,
}

impl Weatherstack {
    #[must_use]
    /// Creates a `Weatherstack` with the specified API key.
    pub fn new(api_key: &str) -> Self {
        Self {
            base_url: "https://api.weatherstack.com/current".to_string(),
            api_key: api_key.to_owned(),
        }
    }

    /// Queries Weatherstack for `location` and returns the weather.
    ///
    /// # Errors
    ///
    /// Returns any errors sending the request, HTTP status errors from the API
    /// server, or errors deserializing the JSON response.
    pub fn get_weather(&self, location: &str) -> Result<Weather> {
        let resp = self.request(location).send()?;
        resp.error_for_status_ref()?;
        deserialize(&resp.text()?)
    }

    fn request(&self, location: &str) -> RequestBuilder {
        reqwest::blocking::Client::new()
            .get(&self.base_url)
            .query(&[("query", location), ("access_key", &self.api_key)])
            .timeout(Duration::from_secs(1))
    }
}

fn deserialize(json: &str) -> Result<Weather> {
    let val: Value = serde_json::from_str(json)?;
    let ctx = format!("bad response: {val}");
    let location_name = val
        .pointer("/location/name")
        .and_then(Value::as_str)
        .with_context(|| ctx.clone())?
        .to_string();
    let location_country = val
        .pointer("/location/country")
        .and_then(Value::as_str)
        .with_context(|| ctx.clone())?;
    let temperature = val
        .pointer("/current/temperature")
        .and_then(Value::as_f64)
        .with_context(|| ctx.clone())?;
    let summary = val
        .pointer("/current/weather_descriptions/0")
        .and_then(Value::as_str)
        .with_context(|| ctx.clone())?
        .to_string();
    Ok(Weather {
        location: location_name + ", " + location_country,
        temperature: Temperature::from_celsius(temperature),
        summary,
    })
}

#[cfg(test)]
mod tests {
    use http::StatusCode;
    use httpmock::{Method, MockServer};
    use url::Host::Domain;

    use std::fs;

    use super::*;

    #[test]
    #[allow(clippy::float_cmp)]
    fn temperature_can_be_expressed_as_celsius_or_fahrenheit() {
        let temp = Temperature::from_celsius(10.0);
        assert_eq!(temp.as_celsius(), 10.0, "wrong celsius");
        assert_eq!(temp.as_fahrenheit(), 50.0, "wrong fahrenheit");
    }

    #[test]
    fn request_builds_correct_request() {
        let ws = Weatherstack::new("dummy API key");
        let req = ws.request("London,UK");
        let req = req.build().unwrap();
        assert_eq!(req.method(), "GET", "wrong method");
        let url = req.url();
        assert_eq!(
            url.host(),
            Some(Domain("api.weatherstack.com")),
            "wrong host"
        );
        assert_eq!(url.path(), "/current", "wrong path");
        let params: Vec<(_, _)> = url.query_pairs().collect();
        assert_eq!(
            params,
            vec![
                ("query".into(), "London,UK".into()),
                ("access_key".into(), "dummy API key".into())
            ],
            "wrong params"
        );
    }

    #[test]
    fn deserialize_extracts_correct_weather_from_json() {
        let json = fs::read_to_string("tests/data/weatherstack.json").unwrap();
        let weather = deserialize(&json).unwrap();
        assert_eq!(
            weather,
            Weather {
                location: "London, United Kingdom".into(),
                temperature: Temperature::from_celsius(11.0),
                summary: "Sunny".into(),
            },
            "wrong weather"
        );
    }

    #[test]
    fn get_weather_fn_makes_correct_api_call() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(Method::GET)
                .path("/current")
                .query_param("query", "London,UK")
                .query_param("access_key", "dummy api key");
            then.status(StatusCode::OK)
                .header("content-type", "application/json")
                .body_from_file("tests/data/weatherstack.json");
        });
        let mut ws = Weatherstack::new("dummy api key");
        ws.base_url = server.base_url() + "/current";
        let weather = ws.get_weather("London,UK");
        mock.assert();
        assert_eq!(
            weather.unwrap(),
            Weather {
                location: "London, United Kingdom".into(),
                temperature: Temperature(11.0),
                summary: "Sunny".into(),
            },
            "wrong weather"
        );
    }
}
