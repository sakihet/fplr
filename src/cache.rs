use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static NO_CACHE: OnceLock<bool> = OnceLock::new();
static TMP_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Set once from the `--no-cache` global flag
pub fn set_no_cache(value: bool) {
    let _ = NO_CACHE.set(value);
}

fn disabled() -> bool {
    *NO_CACHE.get().unwrap_or(&false)
}

pub fn cache_dir() -> Option<PathBuf> {
    let mut path = dirs::cache_dir()?;
    path.push("fplr");
    Some(path)
}

fn now_secs() -> Option<u64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .map(|d| d.as_secs())
}

/// Turn an endpoint into a file name: `/fixtures/?event=2` -> `fixtures_event_2.json`
fn slugify(endpoint: &str) -> String {
    let trimmed = endpoint.trim_matches('/');
    let mut slug = String::with_capacity(trimmed.len() + 5);
    for c in trimmed.chars() {
        if c.is_ascii_alphanumeric() || c == '-' {
            slug.push(c);
        } else if !slug.ends_with('_') {
            slug.push('_');
        }
    }
    slug.push_str(".json");
    slug
}

/// TTL in seconds from a `Cache-Control` header, or None if it must not be stored.
/// `Age` is intentionally ignored: the origin allows staleness via stale-while-revalidate.
fn parse_cache_control(header: &str) -> Option<u64> {
    let mut max_age = None;
    for directive in header.split(',') {
        let directive = directive.trim().to_ascii_lowercase();
        if directive == "no-store" || directive == "no-cache" {
            return None;
        }
        if let Some(value) = directive.strip_prefix("max-age=") {
            max_age = value.trim_matches('"').parse().ok();
        }
    }
    max_age
}

/// Return the cached body if present and unexpired. Any failure is a miss.
pub fn read(endpoint: &str) -> Option<String> {
    if disabled() {
        return None;
    }
    let path = cache_dir()?.join(slugify(endpoint));
    let content = fs::read_to_string(path).ok()?;
    let (expires, body) = content.split_once('\n')?;
    if expires.trim().parse::<u64>().ok()? <= now_secs()? {
        return None;
    }
    Some(body.to_string())
}

/// Store the raw body if the response allows it. Failures are ignored.
pub fn write(endpoint: &str, cache_control: Option<&str>, body: &str) {
    if disabled() {
        return;
    }
    let Some(ttl) = cache_control.and_then(parse_cache_control) else {
        return;
    };
    let (Some(dir), Some(now)) = (cache_dir(), now_secs()) else {
        return;
    };
    if fs::create_dir_all(&dir).is_err() {
        return;
    }

    // Fetches run concurrently, so write to a temp file and rename atomically
    let name = slugify(endpoint);
    let tmp = dir.join(format!(
        ".{}.{}.{}.tmp",
        name,
        std::process::id(),
        TMP_COUNTER.fetch_add(1, Ordering::Relaxed)
    ));
    if fs::write(&tmp, format!("{}\n{}", now + ttl, body)).is_ok() {
        let _ = fs::rename(&tmp, dir.join(name));
    } else {
        let _ = fs::remove_file(&tmp);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slugify_bootstrap_static() {
        assert_eq!(slugify("/bootstrap-static/"), "bootstrap-static.json");
    }

    #[test]
    fn test_slugify_query_string() {
        assert_eq!(slugify("/fixtures/?event=2"), "fixtures_event_2.json");
    }

    #[test]
    fn test_slugify_nested_path() {
        assert_eq!(
            slugify("/entry/123/event/2/picks/"),
            "entry_123_event_2_picks.json"
        );
    }

    #[test]
    fn test_parse_cache_control_max_age() {
        assert_eq!(
            parse_cache_control("max-age=300, stale-while-revalidate=3600"),
            Some(300)
        );
    }

    #[test]
    fn test_parse_cache_control_no_store() {
        assert_eq!(parse_cache_control("no-store"), None);
        assert_eq!(parse_cache_control("max-age=300, no-store"), None);
    }

    #[test]
    fn test_parse_cache_control_no_cache() {
        assert_eq!(parse_cache_control("no-cache, max-age=300"), None);
    }

    #[test]
    fn test_parse_cache_control_missing_max_age() {
        assert_eq!(parse_cache_control("public"), None);
    }

    #[test]
    fn test_parse_cache_control_ignores_s_maxage() {
        assert_eq!(parse_cache_control("public, s-maxage=600"), None);
    }
}
