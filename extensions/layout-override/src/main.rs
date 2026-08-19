use serde::Deserialize;
use std::io::{Read, Write};
use std::time::Duration;
use xfetch_extension_api::{
    ConfigProviderRequest, ConfigProviderResponse, KIND_CONFIG_PROVIDER, with_timeout,
};

#[derive(Debug, Default, Deserialize)]
struct LayoutOverrideArgs {
    layout: Option<String>,
    #[serde(default)]
    modules: Option<Vec<String>>,
}

/// Pure config transformation; 2 s is plenty.
const BUDGET: Duration = Duration::from_secs(2);

fn main() {
    let result = with_timeout(BUDGET, || {
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

        let args: LayoutOverrideArgs = request
            .args
            .as_ref()
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();

        let mut config = request.config.clone();

        if let Some(layout) = &args.layout {
            if let Some(obj) = config.as_object_mut() {
                obj.insert("layout".to_string(), serde_json::json!(layout));
            }
        }

        if let Some(modules) = &args.modules {
            let mods: Vec<serde_json::Value> =
                modules.iter().map(|m| serde_json::json!(m)).collect();
            if let Some(obj) = config.as_object_mut() {
                obj.insert("modules".to_string(), serde_json::Value::Array(mods));
            }
        }

        config
    });

    match result {
        Ok(config) => write_stdout(&ConfigProviderResponse { config }),
        Err(_) => {
            eprintln!("layout-override: timed out");
            std::process::exit(1);
        }
    }
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
