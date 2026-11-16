//! Lua dialects and the syntax features they enable.
//!
//! xmake embeds either Lua 5.4 (the default) or LuaJIT, and future releases
//! may move to newer Lua versions. The grammar always accepts the union of
//! every dialect, so that a file written for another runtime still yields a
//! useful tree; the parser then reports constructs the selected [`Dialect`]
//! does not support, through [`Feature`].
//!
//! Adding a dialect means adding a [`LuaVersion`] variant and listing its
//! features in [`LuaVersion::features`]; adding a construct means adding a
//! [`Feature`] and calling [`crate::parser::Parser::require`] where it is
//! parsed.

use std::fmt;

/// A syntax feature that only some Lua versions understand.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Feature {
    /// `goto name` and `::name::` (Lua 5.2+, LuaJIT).
    Goto,
    /// Integer division `//` (Lua 5.3+).
    IntegerDivision,
    /// Bitwise operators `& | ~ << >>` and unary `~` (Lua 5.3+).
    BitwiseOperators,
    /// Variable attributes `local x <const>` / `<close>` (Lua 5.4+).
    Attributes,
    /// `global` declarations (Lua 5.5+).
    GlobalDeclarations,
}

impl Feature {
    /// Every feature, in declaration order.
    pub const ALL: [Feature; 5] = [
        Feature::Goto,
        Feature::IntegerDivision,
        Feature::BitwiseOperators,
        Feature::Attributes,
        Feature::GlobalDeclarations,
    ];

    /// Human readable name, used in diagnostics.
    pub fn describe(self) -> &'static str {
        match self {
            Feature::Goto => "`goto` and labels",
            Feature::IntegerDivision => "integer division `//`",
            Feature::BitwiseOperators => "bitwise operators",
            Feature::Attributes => "variable attributes `<const>` / `<close>`",
            Feature::GlobalDeclarations => "`global` declarations",
        }
    }

    /// The oldest PUC Lua version supporting the feature.
    pub fn since(self) -> LuaVersion {
        match self {
            Feature::Goto => LuaVersion::Lua52,
            Feature::IntegerDivision | Feature::BitwiseOperators => LuaVersion::Lua53,
            Feature::Attributes => LuaVersion::Lua54,
            Feature::GlobalDeclarations => LuaVersion::Lua55,
        }
    }

    const fn bit(self) -> u16 {
        1 << self as u8
    }
}

/// A set of [`Feature`]s.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Features(u16);

impl Features {
    /// No feature at all: plain Lua 5.1.
    pub const NONE: Features = Features(0);

    /// Builds a set from a list.
    pub const fn of(features: &[Feature]) -> Features {
        let mut bits = 0;
        let mut i = 0;
        while i < features.len() {
            bits |= features[i].bit();
            i += 1;
        }
        Features(bits)
    }

    /// Whether `feature` is in the set.
    pub const fn has(self, feature: Feature) -> bool {
        self.0 & feature.bit() != 0
    }
}

/// A Lua runtime xmake can be built with.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LuaVersion {
    /// Lua 5.1.
    Lua51,
    /// LuaJIT 2.x: Lua 5.1 plus `goto`.
    LuaJit,
    /// Lua 5.2.
    Lua52,
    /// Lua 5.3.
    Lua53,
    /// Lua 5.4, the default runtime of xmake.
    Lua54,
    /// Lua 5.5.
    Lua55,
}

impl LuaVersion {
    /// The syntax features of this version.
    pub const fn features(self) -> Features {
        use Feature::*;
        match self {
            LuaVersion::Lua51 => Features::NONE,
            LuaVersion::LuaJit | LuaVersion::Lua52 => Features::of(&[Goto]),
            LuaVersion::Lua53 => Features::of(&[Goto, IntegerDivision, BitwiseOperators]),
            LuaVersion::Lua54 => {
                Features::of(&[Goto, IntegerDivision, BitwiseOperators, Attributes])
            }
            LuaVersion::Lua55 => Features::of(&[
                Goto,
                IntegerDivision,
                BitwiseOperators,
                Attributes,
                GlobalDeclarations,
            ]),
        }
    }

    /// Parses `5.4`, `lua5.4`, `Lua 5.4`, `luajit`, ... (case-insensitive).
    pub fn parse(s: &str) -> Option<LuaVersion> {
        let s = s.trim();
        let lower_starts = |prefix: &str| {
            s.get(..prefix.len()).is_some_and(|head| head.eq_ignore_ascii_case(prefix))
        };
        if lower_starts("luajit") {
            return Some(LuaVersion::LuaJit);
        }
        let number = if lower_starts("lua") { s[3..].trim_start() } else { s };
        // Ignore a patch level: `5.4.6` is `5.4`.
        let minor = number.strip_prefix("5.")?.split('.').next()?;
        match minor {
            "1" => Some(LuaVersion::Lua51),
            "2" => Some(LuaVersion::Lua52),
            "3" => Some(LuaVersion::Lua53),
            "4" => Some(LuaVersion::Lua54),
            "5" => Some(LuaVersion::Lua55),
            _ => None,
        }
    }
}

impl fmt::Display for LuaVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            LuaVersion::Lua51 => "Lua 5.1",
            LuaVersion::LuaJit => "LuaJIT",
            LuaVersion::Lua52 => "Lua 5.2",
            LuaVersion::Lua53 => "Lua 5.3",
            LuaVersion::Lua54 => "Lua 5.4",
            LuaVersion::Lua55 => "Lua 5.5",
        })
    }
}

/// The language a file is parsed with.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Dialect {
    /// The Lua runtime.
    pub lua: LuaVersion,
}

impl Dialect {
    /// The dialect of the default xmake runtime.
    pub const XMAKE_DEFAULT: Dialect = Dialect { lua: LuaVersion::Lua54 };

    /// Whether the dialect supports `feature`.
    pub const fn has(self, feature: Feature) -> bool {
        self.lua.features().has(feature)
    }
}

impl Default for Dialect {
    fn default() -> Dialect {
        Dialect::XMAKE_DEFAULT
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_versions() {
        assert_eq!(LuaVersion::parse("5.4"), Some(LuaVersion::Lua54));
        assert_eq!(LuaVersion::parse("Lua 5.3.6"), Some(LuaVersion::Lua53));
        assert_eq!(LuaVersion::parse("LuaJIT 2.1.0"), Some(LuaVersion::LuaJit));
        assert_eq!(LuaVersion::parse("lua5.1"), Some(LuaVersion::Lua51));
        assert_eq!(LuaVersion::parse("6.0"), None);
    }

    #[test]
    fn features_grow_with_versions() {
        for feature in Feature::ALL {
            assert!(feature.since().features().has(feature), "{feature:?}");
            assert!(LuaVersion::Lua55.features().has(feature), "{feature:?}");
            assert!(!LuaVersion::Lua51.features().has(feature), "{feature:?}");
        }
        assert!(LuaVersion::LuaJit.features().has(Feature::Goto));
        assert!(!LuaVersion::LuaJit.features().has(Feature::IntegerDivision));
    }
}
