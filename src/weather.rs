use std::fmt::Display;

use crate::temperature::{Celsius, Temperature};

#[derive(Debug, PartialEq)]
pub struct Weather {
    pub location: String,
    pub temperature: Temperature,
    pub summary: String,
}

impl Display for Weather {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}ºC ({})",
            self.summary,
            self.temperature.to::<Celsius>(),
            self.location
        )
    }
}

pub trait Provider {
    /// Gets the current weather conditions from the provider.
    /// 
    /// # Errors
    /// 
    /// Any errors from the HTTP API request, or deserializing the response JSON
    /// into a `Weather`.
    fn get_weather(&self, location: &str) -> anyhow::Result<Weather>;
}
