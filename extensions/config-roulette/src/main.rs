use serde::Deserialize;
use std::io::{Read, Write};
use xfetch_extension_api::{ConfigProviderRequest, ConfigProviderResponse, KIND_CONFIG_PROVIDER};

#[derive(Debug, Deserialize)]
struct Route {
    #[serde(default)]
    _name: String,
    path: String,
}

#[derive(Debug, Deserialize)]
struct RouletteArgs {
    routes: String,
    #[serde(default = "default_strategy")]
    strategy: Strategy,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Strategy {
    Random,
    Daily,
}

fn default_strategy() -> Strategy {
    Strategy::Daily
}

fn main() {
    let request: ConfigProviderRequest = match read_stdin() {
        Ok(v) => v,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    if request.kind != KIND_CONFIG_PROVIDER {
        eprintln!("Unsupported kind: {}", request.kind);
        std::process::exit(1);
    }

    let args: RouletteArgs = request
        .args
        .as_ref()
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_else(|| RouletteArgs {
            routes: "~/.config/xfetch/routes.json".to_string(),
            strategy: default_strategy(),
        });

    let routes_path = expand_path(&args.routes);
    let content = match std::fs::read_to_string(&routes_path) {
        Ok(c) => c,
        Err(err) => {
            eprintln!("Failed to read routes file '{}': {}", routes_path.display(), err);
            std::process::exit(1);
        }
    };

    let all_routes: Vec<Route> = match serde_json::from_str(&content) {
        Ok(r) => r,
        Err(err) => {
            eprintln!("Failed to parse routes file: {}", err);
            std::process::exit(1);
        }
    };

    if all_routes.is_empty() {
        eprintln!("Routes file is empty");
        std::process::exit(1);
    }

    let index = match args.strategy {
        Strategy::Random => {
            let nanos = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .subsec_nanos();
            (nanos as usize) % all_routes.len()
        }
        Strategy::Daily => {
            let now = chrono_day();
            (now as usize) % all_routes.len()
        }
    };

    let chosen = &all_routes[index];
    let config_path = expand_path(&chosen.path);

    let config_content = match std::fs::read_to_string(&config_path) {
        Ok(c) => c,
        Err(err) => {
            eprintln!("Failed to read config '{}': {}", config_path.display(), err);
            std::process::exit(1);
        }
    };

    let config: serde_json::Value = match serde_json::from_str(&config_content) {
        Ok(v) => v,
        Err(err) => {
            eprintln!("Failed to parse config '{}': {}", config_path.display(), err);
            std::process::exit(1);
        }
    };

    write_stdout(&ConfigProviderResponse { config });
}

fn chrono_day() -> u32 {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    (secs / 86400) as u32
}

fn expand_path(path: &str) -> std::path::PathBuf {
    if let Some(rest) = path.strip_prefix('~') {
        if let Ok(home) = std::env::var("HOME") {
            return std::path::PathBuf::from(home).join(rest.strip_prefix('/').unwrap_or(rest));
        }
    }
    std::path::PathBuf::from(path)
}

fn read_stdin<T: serde::de::DeserializeOwned>() -> Result<T, String> {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .map_err(|e| format!("Failed to read stdin: {}", e))?;
    serde_json::from_str(&input).map_err(|e| format!("Failed to parse request: {}", e))
}

fn write_stdout<T: serde::Serialize>(value: &T) {
    let body = serde_json::to_vec(value).expect("Failed to serialize response");
    let mut stdout = std::io::stdout();
    stdout.write_all(&body).expect("Failed to write response");
    stdout.flush().expect("Failed to flush stdout");
}
