//! "Did you mean ...?" suggestions.

/// The candidate closest to `word`, if one is close enough to be a typo:
/// at most a third of the letters differ (one at least), or `word` is an
/// abbreviation (`bin` for `binary`).
pub fn closest<'c>(word: &str, candidates: impl IntoIterator<Item = &'c str>) -> Option<&'c str> {
    let limit = (word.chars().count() / 3).max(1);
    let mut best: Option<(usize, &str)> = None;
    for candidate in candidates {
        if candidate == word {
            continue;
        }
        let is_prefix = word.len() >= 2
            && candidate.get(..word.len()).is_some_and(|head| head.eq_ignore_ascii_case(word));
        // An abbreviation ranks after real typos, shorter completions first.
        let distance = if is_prefix {
            limit + candidate.len() - word.len()
        } else {
            levenshtein(word, candidate)
        };
        let limit = if is_prefix { usize::MAX } else { limit };
        if distance <= limit && best.is_none_or(|(d, _)| distance < d) {
            best = Some((distance, candidate));
        }
    }
    best.map(|(_, c)| c)
}

/// Edit distance, case-insensitive, a swap of two neighbors counting as one
/// edit (`zlbi` is one edit from `zlib`). Words are short: three rows suffice.
fn levenshtein(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().map(|c| c.to_ascii_lowercase()).collect();
    let b: Vec<char> = b.chars().map(|c| c.to_ascii_lowercase()).collect();
    let mut before: Vec<usize> = vec![0; b.len() + 1];
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    let mut cur = vec![0; b.len() + 1];
    for i in 0..a.len() {
        cur[0] = i + 1;
        for j in 0..b.len() {
            let substitution = prev[j] + usize::from(a[i] != b[j]);
            let mut best = substitution.min(prev[j + 1] + 1).min(cur[j] + 1);
            if i > 0 && j > 0 && a[i] == b[j - 1] && a[i - 1] == b[j] {
                best = best.min(before[j - 1] + 1);
            }
            cur[j + 1] = best;
        }
        std::mem::swap(&mut before, &mut prev);
        std::mem::swap(&mut prev, &mut cur);
    }
    prev[b.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suggestions() {
        assert_eq!(levenshtein("kitten", "sitting"), 3);
        assert_eq!(levenshtein("zlbi", "zlib"), 1);
        assert_eq!(closest("set_knd", ["set_kind", "set_default"]), Some("set_kind"));
        assert_eq!(closest("bin", ["binary", "static"]), Some("binary"));
        assert_eq!(closest("xyz", ["binary", "static"]), None);
        assert_eq!(closest("statc", ["binary", "static"]), Some("static"));
    }
}
