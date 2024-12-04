pub trait Unit {
    const ZERO_CENTIGRADE: f32 = 273.15;
    fn to_kelvin(val: impl Into<f32>) -> f32;
    fn from_kelvin(val: f32) -> f32;
}

pub struct Celsius;
pub struct Fahrenheit;

impl Unit for Celsius {
    fn to_kelvin(val: impl Into<f32>) -> f32 {
        val.into() + Self::ZERO_CENTIGRADE
    }

    fn from_kelvin(val: f32) -> f32 {
        val - Self::ZERO_CENTIGRADE
    }
}

impl Unit for Fahrenheit {
    fn to_kelvin(val: impl Into<f32>) -> f32 {
        (val.into() - 32.) / 1.8 + 273.15
    }

    fn from_kelvin(val: f32) -> f32 {
        (val - 273.15) * 1.8 + 32.
    }
}

#[derive(Debug, PartialEq)]
pub struct Temperature(f32);

impl Temperature {
    pub fn from<U: Unit>(val: impl Into<f32>) -> Self {
        Self(U::to_kelvin(val))
    }

    #[must_use]
    pub fn to<U: Unit>(&self) -> f32 {
        U::from_kelvin(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::float_cmp)]
    fn temperature_converts_between_units_correctly() {
        let t = Temperature::from::<Celsius>(0.);
        assert_eq!(t.to::<Celsius>(), 0.);
        assert_eq!(t.to::<Fahrenheit>(), 32.);
        let t = Temperature::from::<Fahrenheit>(212.);
        assert_eq!(t.to::<Fahrenheit>(), 212.);
        assert_eq!(t.to::<Celsius>(), 100.);
    }
}
