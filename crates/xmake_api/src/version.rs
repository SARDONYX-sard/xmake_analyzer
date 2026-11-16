//! xmake versions.

use std::fmt;

/// A `major.minor.patch` version.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl Version {
    /// Creates a version.
    pub const fn new(major: u16, minor: u16, patch: u16) -> Version {
        Version { major, minor, patch }
    }

    /// Parses `3.1.1`, `v3.1.1+HEAD.3ba37a0d4` or `2.8` (missing parts are 0).
    pub fn parse(s: &str) -> Option<Version> {
        let s = s.trim();
        let s = s.strip_prefix('v').unwrap_or(s);
        // Drop build metadata and pre-release tags.
        let core = s.split(['+', '-', ' ']).next()?;
        let mut parts = core.split('.').map(|p| p.parse::<u16>());
        let major = parts.next()?.ok()?;
        let minor = parts.next().unwrap_or(Ok(0)).ok()?;
        let patch = parts.next().unwrap_or(Ok(0)).ok()?;
        Some(Version { major, minor, patch })
    }

    /// Finds the version in the output of `xmake --version`:
    /// `xmake v3.1.1+HEAD.3ba37a0d4, A cross-platform build utility ...`.
    pub fn from_xmake_banner(banner: &str) -> Option<Version> {
        banner
            .split_whitespace()
            .find(|w| w.starts_with('v'))
            .and_then(|w| Version::parse(w.trim_end_matches(',')))
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!(Version::parse("3.1.1"), Some(Version::new(3, 1, 1)));
        assert_eq!(Version::parse("v2.8"), Some(Version::new(2, 8, 0)));
        assert_eq!(Version::parse("x"), None);
        let banner = "\x1b[1mxmake v3.1.1+HEAD.3ba37a0d4, A cross-platform build utility";
        assert_eq!(Version::from_xmake_banner(banner), Some(Version::new(3, 1, 1)));
    }
}
