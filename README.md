[![Crate](https://img.shields.io/crates/v/umbrella.svg)](https://crates.io/crates/umbrella)
[![Docs](https://docs.rs/umbrella/badge.svg)](https://docs.rs/umbrella)
![CI](https://github.com/bitfield/umbrella/actions/workflows/ci.yml/badge.svg)
![Audit](https://github.com/bitfield/umbrella/actions/workflows/audit.yml/badge.svg)
![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

# umbrella

A simple weather reporting tool in Rust, using the Weatherstack API.

# Installation

```sh
cargo install umbrella
```

# Usage

First, create a [Weatherstack account](https://weatherstack.com/signup/free), and obtain your API key.

```sh
export WEATHERSTACK_API_KEY=xxx
umbrella London,UK
```

```
Sunny 4.0ºC (London, United Kingdom)
```

# Units

To report temperature in Fahrenheit, use the `--fahrenheit` (or `-f`) flag:

```sh
umbrella --fahrenheit Los Angeles, USA
```

```
Fog 50.0ºF (Los Angeles, United States of America)
```
