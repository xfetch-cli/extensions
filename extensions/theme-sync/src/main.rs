//! `theme-sync`: apply the active equisdots palette to an xfetch config.
//!
//! The extension reads `settings.json` (`bar.palette`, with `dock.palette` as
//! legacy fallback), loads `themes/<slug>.jsonc`, merges its colors and logo
//! accent into the config and sets the active theme. It is a no-op when the
//! config already carries a non-empty `colors` object unless `force` is set;
//! this is what makes it useful after `config-roulette`: the selected route
//! config has no colors, so theme-sync restores them at runtime.

use serde::Deserialize;
use serde_json::{Map, Value};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;
use xfetch_extension_api::{ConfigProviderRequest, ConfigProviderResponse, with_timeout};

const BUDGET: Duration = Duration::from_secs(3);
const CONFIG_MOUNT: &str = "/config";

#[derive(Debug, Deserialize)]
struct ThemeSyncArgs {
    #[serde(default = "default_settings")]
    settings: String,
    #[serde(default = "default_themes_dir")]
    themes_dir: String,
    #[serde(default = "default_fallback")]
    fallback: String,
    #[serde(default)]
    force: bool,
}

impl Default for ThemeSyncArgs {
    fn default() -> Self {
        Self {
            settings: default_settings(),
            themes_dir: default_themes_dir(),
            fallback: default_fallback(),
            force: false,
        }
    }
}

fn default_settings() -> String {
    "~/.config/hypr/settings.json".to_string()
}

fn default_themes_dir() -> String {
    "~/.config/xfetch/themes".to_string()
}

fn default_fallback() -> String {
    "x".to_string()
}

fn main() {
    match with_timeout(BUDGET, run) {
        Ok(Ok(response)) => write_response(&response),
        Ok(Err(err)) => {
            eprintln!("theme-sync: {err}");
            std::process::exit(1);
        }
        Err(_) => {
            eprintln!("theme-sync: timed out");
            std::process::exit(1);
        }
    }
}

fn run() -> Result<ConfigProviderResponse, String> {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .map_err(|err| format!("failed to read request: {err}"))?;
    let request: ConfigProviderRequest =
        serde_json::from_str(&input).map_err(|err| format!("invalid request: {err}"))?;
    request
        .validate()
        .map_err(|err| format!("invalid request: {err}"))?;

    let args = request
        .args
        .as_ref()
        .and_then(|value| serde_json::from_value(value.clone()).ok())
        .unwrap_or_default();

    Ok(ConfigProviderResponse {
        config: apply(&request.config, &args),
    })
}

fn write_response(response: &ConfigProviderResponse) {
    let body = serde_json::to_vec(response).expect("failed to serialize response");
    let mut stdout = std::io::stdout();
    stdout.write_all(&body).expect("failed to write response");
    stdout.flush().expect("failed to flush response");
}

fn apply(config: &Value, args: &ThemeSyncArgs) -> Value {
    let mut config = config.clone();

    if !args.force && has_colors(&config) {
        return config;
    }

    let settings = read_json(&map_path(&args.settings)).unwrap_or(Value::Null);
    let slug = active_slug(&settings, &args.fallback);
    let theme_path = map_path(&args.themes_dir).join(format!("{slug}.jsonc"));
    let Some(theme) = read_jsonc(&theme_path) else {
        return config;
    };

    merge_theme(&mut config, &theme, &slug, args.force);
    config
}

fn has_colors(config: &Value) -> bool {
    config
        .get("colors")
        .and_then(Value::as_object)
        .is_some_and(|colors| !colors.is_empty())
}

fn active_slug(settings: &Value, fallback: &str) -> String {
    for section in ["bar", "dock"] {
        let slug = settings
            .get(section)
            .and_then(|value| value.get("palette"))
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|slug| !slug.is_empty());
        if let Some(slug) = slug {
            return slug.to_lowercase();
        }
    }
    fallback.trim().to_lowercase()
}

fn merge_theme(config: &mut Value, theme: &Value, slug: &str, force: bool) {
    let theme_colors = theme
        .get("colors")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    let mut merged = config
        .get("colors")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();

    for (key, value) in &theme_colors {
        merged.insert(key.clone(), value.clone());
    }

    if !theme_colors.is_empty() {
        add_plugin_fallbacks(&mut merged, theme);
    }
    if !merged.is_empty() {
        config["colors"] = Value::Object(merged);
    }

    if let Some(logo_color) = theme.get("logo_color")
        && (force || config.get("logo_color").is_none())
    {
        config["logo_color"] = logo_color.clone();
    }
    config["theme"] = Value::String(slug.to_string());
}

fn add_plugin_fallbacks(colors: &mut Map<String, Value>, theme: &Value) {
    if !colors.contains_key("plugin:gauges") {
        let fallback = colors
            .get("memory")
            .or_else(|| colors.get("disk"))
            .or_else(|| colors.values().next())
            .cloned();
        if let Some(value) = fallback {
            colors.insert("plugin:gauges".to_string(), value);
        }
    }

    if !colors.contains_key("plugin:banner-text") {
        let fallback = theme
            .get("logo_color")
            .cloned()
            .or_else(|| colors.get("memory").cloned())
            .or_else(|| colors.values().next().cloned());
        if let Some(value) = fallback {
            colors.insert("plugin:banner-text".to_string(), value);
        }
    }
}

fn read_json(path: &Path) -> Option<Value> {
    let text = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&text).ok()
}

fn read_jsonc(path: &Path) -> Option<Value> {
    let text = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&strip_jsonc(&text)).ok()
}

fn map_path(value: &str) -> PathBuf {
    let value = value.trim();

    if let Some(rest) = value.strip_prefix("~/.config") {
        return Path::new(CONFIG_MOUNT).join(rest.trim_start_matches('/'));
    }

    if let Some(home) = std::env::var_os("HOME") {
        let config_dir = Path::new(&home).join(".config");
        if let Ok(rest) = Path::new(value).strip_prefix(&config_dir) {
            return Path::new(CONFIG_MOUNT).join(rest);
        }
    }

    PathBuf::from(value)
}

fn strip_jsonc(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    let mut in_string = false;
    let mut escaped = false;

    while let Some(ch) = chars.next() {
        if in_string {
            out.push(ch);
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }

        match ch {
            '"' => {
                in_string = true;
                out.push(ch);
            }
            '/' if chars.peek() == Some(&'/') => {
                chars.next();
                for next in chars.by_ref() {
                    if next == '\n' || next == '\r' {
                        out.push('\n');
                        break;
                    }
                }
            }
            '/' if chars.peek() == Some(&'*') => {
                chars.next();
                let mut prev = '\0';
                for next in chars.by_ref() {
                    if prev == '*' && next == '/' {
                        break;
                    }
                    prev = next;
                }
            }
            other => out.push(other),
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn strips_comments_without_touching_strings() {
        let input = r#"{
            "url": "https://example.com/a//b",
            "a": 1, // line comment
            /* block comment */
            "b": 2
        }"#;
        let value: Value = serde_json::from_str(&strip_jsonc(input)).unwrap();
        assert_eq!(value["url"], "https://example.com/a//b");
        assert_eq!(value["a"], 1);
        assert_eq!(value["b"], 2);
    }

    #[test]
    fn reads_bar_then_dock_then_fallback() {
        let bar = json!({"bar": {"palette": " Chalk "}});
        let dock = json!({"dock": {"palette": "x"}});
        assert_eq!(active_slug(&bar, "x"), "chalk");
        assert_eq!(active_slug(&dock, "chalk"), "x");
        assert_eq!(active_slug(&json!({}), "x"), "x");
    }

    #[test]
    fn merges_colors_and_plugin_fallbacks() {
        let mut config = json!({"theme": "old"});
        let theme = json!({
            "colors": {
                "cpu": "#111111",
                "memory": "#222222",
                "disk": "#333333"
            },
            "logo_color": "#444444"
        });
        merge_theme(&mut config, &theme, "chalk", false);

        assert_eq!(config["theme"], "chalk");
        assert_eq!(config["colors"]["cpu"], "#111111");
        assert_eq!(config["colors"]["plugin:gauges"], "#222222");
        assert_eq!(config["colors"]["plugin:banner-text"], "#444444");
        assert_eq!(config["logo_color"], "#444444");
    }

    #[test]
    fn detects_existing_colors() {
        assert!(!has_colors(&json!({})));
        assert!(!has_colors(&json!({"colors": {}})));
        assert!(has_colors(&json!({"colors": {"cpu": "#111111"}})));
    }

    #[test]
    fn applies_theme_from_files() {
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir =
            std::env::temp_dir().join(format!("theme-sync-test-{}-{stamp}", std::process::id()));
        let themes = dir.join("themes");
        std::fs::create_dir_all(&themes).unwrap();
        std::fs::write(dir.join("settings.json"), r#"{"bar":{"palette":"chalk"}}"#).unwrap();
        std::fs::write(
            themes.join("chalk.jsonc"),
            "// generated\n{\"colors\":{\"cpu\":\"#111111\"},\"logo_color\":\"#222222\"}\n",
        )
        .unwrap();

        let args = ThemeSyncArgs {
            settings: dir.join("settings.json").to_string_lossy().into_owned(),
            themes_dir: themes.to_string_lossy().into_owned(),
            fallback: "x".to_string(),
            force: false,
        };
        let config = apply(&json!({}), &args);
        assert_eq!(config["theme"], "chalk");
        assert_eq!(config["colors"]["cpu"], "#111111");
        assert_eq!(config["logo_color"], "#222222");

        let kept = apply(&json!({"colors": {"cpu": "#123456"}}), &args);
        assert_eq!(kept["colors"]["cpu"], "#123456");
        assert!(kept.get("theme").is_none());

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn maps_config_paths_into_the_guest_mount() {
        assert_eq!(
            map_path("~/.config/hypr/settings.json"),
            Path::new("/config/hypr/settings.json")
        );
        assert_eq!(
            map_path("~/.config/xfetch/themes"),
            Path::new("/config/xfetch/themes")
        );
        assert_eq!(
            map_path("/config/xfetch/themes"),
            Path::new("/config/xfetch/themes")
        );
    }
}
