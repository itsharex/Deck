mod locales;

use std::sync::OnceLock;

static LOCALE: OnceLock<String> = OnceLock::new();

/// Detect the macOS system locale and initialize translations.
pub fn init() {
    let locale = detect_macos_locale();
    LOCALE.set(locale).ok();
}

/// Get the current locale identifier.
pub fn locale() -> &'static str {
    LOCALE.get().map(|s| s.as_str()).unwrap_or("zh-Hans")
}

/// Look up a translated string by key.
/// Falls back to zh-Hans (source language), then to the raw key.
pub fn t(key: &str) -> String {
    let loc = locale();
    locales::get(loc, key)
        .or_else(|| locales::get("zh-Hans", key))
        .unwrap_or_else(|| key.to_string())
}

/// Detect macOS system locale from global preferences.
fn detect_macos_locale() -> String {
    // Allow explicit override via environment variable (for testing/debugging)
    if let Ok(val) = std::env::var("DECKCLIP_LANG") {
        if let Some(lang) = map_to_supported_locale(&val) {
            return lang.to_string();
        }
    }

    // Try reading AppleLanguages from macOS global preferences
    if let Ok(output) = std::process::Command::new("defaults")
        .args(["read", "-g", "AppleLanguages"])
        .output()
    {
        if output.status.success() {
            let text = String::from_utf8_lossy(&output.stdout);
            // Parse plist array format: ( "zh-Hans-CN", "en-US", ... )
            for line in text.lines() {
                let trimmed = line.trim().trim_matches(|c| c == '"' || c == ',');
                if let Some(lang) = map_to_supported_locale(trimmed) {
                    return lang.to_string();
                }
            }
        }
    }

    // Fallback: check LANG / LC_ALL environment variables
    for var in &["LC_ALL", "LANG", "LC_MESSAGES"] {
        if let Ok(val) = std::env::var(var) {
            if let Some(lang) = map_to_supported_locale(&val) {
                return lang.to_string();
            }
        }
    }

    "zh-Hans".to_string()
}

/// Map a locale identifier (e.g., "zh-Hans-CN", "en-US", "de_DE.UTF-8")
/// to one of our supported locales.
fn map_to_supported_locale(raw: &str) -> Option<&'static str> {
    let lower = raw.to_lowercase().replace('_', "-");
    let lower = lower.trim_end_matches(".utf-8").trim_end_matches(".utf8");

    if lower.starts_with("zh-hant") || lower.starts_with("zh-tw") || lower.starts_with("zh-hk") {
        Some("zh-Hant")
    } else if lower.starts_with("zh-hans") || lower.starts_with("zh-cn") || lower == "zh" {
        Some("zh-Hans")
    } else if lower.starts_with("en") {
        Some("en")
    } else if lower.starts_with("de") {
        Some("de")
    } else if lower.starts_with("fr") {
        Some("fr")
    } else if lower.starts_with("ja") {
        Some("ja")
    } else if lower.starts_with("ko") {
        Some("ko")
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locale_mapping() {
        assert_eq!(map_to_supported_locale("zh-Hans-CN"), Some("zh-Hans"));
        assert_eq!(map_to_supported_locale("zh-Hant-TW"), Some("zh-Hant"));
        assert_eq!(map_to_supported_locale("en-US"), Some("en"));
        assert_eq!(map_to_supported_locale("en_GB.UTF-8"), Some("en"));
        assert_eq!(map_to_supported_locale("de_DE"), Some("de"));
        assert_eq!(map_to_supported_locale("fr-FR"), Some("fr"));
        assert_eq!(map_to_supported_locale("ja"), Some("ja"));
        assert_eq!(map_to_supported_locale("ko-KR"), Some("ko"));
        assert_eq!(map_to_supported_locale("("), None);
        assert_eq!(map_to_supported_locale(")"), None);
    }
}
