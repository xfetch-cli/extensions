//! `wasm-night-mode`: a WebAssembly config provider that dims the fetch at
//! night.
//!
//! Outside the configured daytime window it turns colors off (and can switch
//! the palette style), so late-night terminals are easier on the eyes. The
//! same source builds natively for debugging and as a `wasm32-wasip1` guest.

use serde::Deserialize;
use serde_json::Value;
use std::io::{Read, Write};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use xfetch_extension_api::{
    ConfigProviderRequest, ConfigProviderResponse, KIND_CONFIG_PROVIDER, with_timeout,
};

/// Config providers should be fast; reading the clock is instant.
const BUDGET: Duration = Duration::from_secs(5);

/// Optional `args` accepted from the config.
#[derive(Debug, Deserialize)]
struct NightArgs {
    /// First hour (inclusive) of the night window, 0-23.
    #[serde(default = "default_night_start")]
    night_start: u32,
    /// First hour (inclusive) of the day window, 0-23.
    #[serde(default = "default_night_end")]
    night_end: u32,
    /// Offset from UTC in hours, used to compute the local hour.
    #[serde(default)]
    utc_offset: i32,
    /// Palette style applied at night (default: keep the configured one).
    night_palette: Option<String>,
    /// When false, colors stay enabled at night.
    #[serde(default = "default_true")]
    dim_colors: bool,
}

fn default_night_start() -> u32 {
    22
}

fn default_night_end() -> u32 {
    7
}

fn default_true() -> bool {
    true
}

impl Default for NightArgs {
    fn default() -> Self {
        Self {
            night_start: default_night_start(),
            night_end: default_night_end(),
            utc_offset: 0,
            night_palette: None,
            dim_colors: true,
        }
    }
}

fn main() {
    let response = match with_timeout(BUDGET, run) {
        Ok(Ok(response)) => response,
        Ok(Err(err)) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
        Err(_) => {
            eprintln!("wasm-night-mode: timed out");
            std::process::exit(1);
        }
    };

    let body = serde_json::to_vec(&response).expect("Failed to serialize response");
    let mut stdout = std::io::stdout();
    stdout.write_all(&body).expect("Failed to write response");
    stdout.flush().expect("Failed to flush stdout");
}

/// Reads the request, transforms the config and returns the response.
fn run() -> Result<ConfigProviderResponse, String> {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .map_err(|err| format!("Failed to read stdin: {}", err))?;
    let request: ConfigProviderRequest =
        serde_json::from_str(&input).map_err(|err| format!("Failed to parse request: {}", err))?;

    if request.kind != KIND_CONFIG_PROVIDER {
        return Err(format!("Unsupported kind: {}", request.kind));
    }

    let args: NightArgs = request
        .args
        .as_ref()
        .and_then(|value| serde_json::from_value(value.clone()).ok())
        .unwrap_or_default();

    let hour = local_hour(args.utc_offset);
    let is_night = if args.night_start <= args.night_end {
        (args.night_start..args.night_end).contains(&hour)
    } else {
        hour >= args.night_start || hour < args.night_end
    };

    let mut config = request.config;
    if is_night {
        if args.dim_colors {
            config["show_colors"] = Value::Bool(false);
        }
        if let Some(palette) = &args.night_palette {
            config["palette_style"] = Value::String(palette.clone());
        }
    }

    Ok(ConfigProviderResponse { config })
}

/// Current hour in the configured offset, without needing a timezone database.
fn local_hour(utc_offset: i32) -> u32 {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;
    let shifted = seconds + i64::from(utc_offset) * 3600;
    (shifted.div_euclid(3600) % 24) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn night_window_wraps_midnight() {
        let args = NightArgs::default();
        assert!(is_night_for_hour(&args, 23));
        assert!(is_night_for_hour(&args, 3));
        assert!(!is_night_for_hour(&args, 12));
    }

    #[test]
    fn plain_window_within_a_day() {
        let args = NightArgs {
            night_start: 1,
            night_end: 5,
            ..NightArgs::default()
        };
        assert!(is_night_for_hour(&args, 3));
        assert!(!is_night_for_hour(&args, 6));
        assert!(!is_night_for_hour(&args, 0));
    }

    fn is_night_for_hour(args: &NightArgs, hour: u32) -> bool {
        if args.night_start <= args.night_end {
            (args.night_start..args.night_end).contains(&hour)
        } else {
            hour >= args.night_start || hour < args.night_end
        }
    }
}
