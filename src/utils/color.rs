use std::ffi::OsString;
use std::fmt::{self, Display, Formatter};
use std::io::IsTerminal;
use std::sync::OnceLock;

use owo_colors::OwoColorize;

static ENABLED: OnceLock<bool> = OnceLock::new();

/// Decide once whether colored output is allowed, following the conventions at
/// <https://clig.dev/#output>. Color is suppressed when any of these hold:
/// the `--no-color` flag is passed, `NO_COLOR` or `FPLR_NO_COLOR` is set to a
/// non-empty value, `TERM` is `dumb`, or stdout is not a terminal.
pub fn init(no_color_flag: bool) {
    let _ = ENABLED.set(detect(no_color_flag));
}

fn detect(no_color_flag: bool) -> bool {
    if no_color_flag {
        return false;
    }
    if disables(std::env::var_os("NO_COLOR")) || disables(std::env::var_os("FPLR_NO_COLOR")) {
        return false;
    }
    if term_is_dumb(std::env::var_os("TERM")) {
        return false;
    }
    std::io::stdout().is_terminal()
}

/// `NO_COLOR` only counts when set to a non-empty value, so exporting it empty
/// leaves color on.
fn disables(value: Option<OsString>) -> bool {
    value.is_some_and(|v| !v.is_empty())
}

fn term_is_dumb(value: Option<OsString>) -> bool {
    value.is_some_and(|v| v == "dumb")
}

/// Whether colored output is enabled. Defaults to off when `init` was never
/// called, so non-CLI callers such as tests stay plain.
pub fn enabled() -> bool {
    *ENABLED.get().unwrap_or(&false)
}

#[derive(Clone, Copy)]
enum Paint {
    Bold,
    BrightYellow,
    Cyan,
    DefaultColor,
    Dimmed,
    Green,
    Red,
    Yellow,
}

/// A value tagged with a style, rendered only when color is enabled.
///
/// The `Display` impl forwards the formatter to the wrapped value, so width and
/// precision specifiers still apply to the text rather than to the escape
/// sequences — the same way `owo_colors` wrappers behave.
pub struct Painted<'a, T: ?Sized> {
    inner: &'a T,
    paint: Paint,
}

impl<T: Display + ?Sized> Display for Painted<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if !enabled() {
            return self.inner.fmt(f);
        }
        match self.paint {
            Paint::Bold => Display::fmt(&OwoColorize::bold(&self.inner), f),
            Paint::BrightYellow => Display::fmt(&OwoColorize::bright_yellow(&self.inner), f),
            Paint::Cyan => Display::fmt(&OwoColorize::cyan(&self.inner), f),
            Paint::DefaultColor => Display::fmt(&OwoColorize::default_color(&self.inner), f),
            Paint::Dimmed => Display::fmt(&OwoColorize::dimmed(&self.inner), f),
            Paint::Green => Display::fmt(&OwoColorize::green(&self.inner), f),
            Paint::Red => Display::fmt(&OwoColorize::red(&self.inner), f),
            Paint::Yellow => Display::fmt(&OwoColorize::yellow(&self.inner), f),
        }
    }
}

/// Drop-in replacement for `owo_colors::OwoColorize` that emits plain text when
/// color is disabled. Calls chain the same way they do with `owo_colors`.
pub trait Colorize: Display {
    fn bold(&self) -> Painted<'_, Self>;
    fn bright_yellow(&self) -> Painted<'_, Self>;
    fn cyan(&self) -> Painted<'_, Self>;
    fn default_color(&self) -> Painted<'_, Self>;
    fn dimmed(&self) -> Painted<'_, Self>;
    fn green(&self) -> Painted<'_, Self>;
    fn red(&self) -> Painted<'_, Self>;
    fn yellow(&self) -> Painted<'_, Self>;
}

macro_rules! colorize_methods {
    ($($name:ident => $paint:ident),* $(,)?) => {
        $(
            fn $name(&self) -> Painted<'_, Self> {
                Painted { inner: self, paint: Paint::$paint }
            }
        )*
    };
}

impl<T: Display + ?Sized> Colorize for T {
    colorize_methods!(
        bold => Bold,
        bright_yellow => BrightYellow,
        cyan => Cyan,
        default_color => DefaultColor,
        dimmed => Dimmed,
        green => Green,
        red => Red,
        yellow => Yellow,
    );
}

#[cfg(test)]
mod tests {
    // Imported by name rather than with a glob: a glob would also pull in
    // `OwoColorize`, making every `.green()` call below ambiguous.
    use super::{Colorize, detect, disables, enabled, term_is_dumb};
    use std::ffi::OsString;

    #[test]
    fn test_disables_only_on_non_empty_value() {
        assert!(!disables(None));
        assert!(!disables(Some(OsString::from(""))));
        assert!(disables(Some(OsString::from("1"))));
        assert!(disables(Some(OsString::from("0"))));
    }

    #[test]
    fn test_term_is_dumb() {
        assert!(!term_is_dumb(None));
        assert!(!term_is_dumb(Some(OsString::from("xterm-256color"))));
        assert!(term_is_dumb(Some(OsString::from("dumb"))));
    }

    #[test]
    fn test_flag_wins_over_terminal_detection() {
        assert!(!detect(true));
    }

    #[test]
    fn test_painted_is_plain_while_disabled() {
        // `init` is never called in tests, so color stays off
        assert!(!enabled());
        assert_eq!("W".green().to_string(), "W");
        assert_eq!(format!("{:<5}|", "ID".bold()), "ID   |");
        assert_eq!(format!("{:.1}", 61.44_f64.bold().cyan()), "61.4");
    }
}
