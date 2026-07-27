// The one JSON primitive grove owns: escaping a string into a JSON string body.
//
// grove carries **no JSON serialiser**, deliberately — it emits exactly two
// fixed-shape documents, herdr's report envelope (`herdr.rs`) and claude's
// injected hook settings (`launch.rs`), both of which are string fields with a
// vocabulary grove itself controls. What grove does *not* control in either is
// the one value that comes from outside it: herdr's opaque pane handle, and the
// filesystem path of `grove-llm`. Those are escaped rather than assumed
// well-formed, and both writers escape them the same way, which is why this
// lives here rather than twice.

/// Escape `raw` for inclusion between the quotes of a JSON string.
pub fn escape(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    for c in raw.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_two_structural_characters_are_escaped() {
        assert_eq!(escape(r#"a"b\c"#), r#"a\"b\\c"#);
    }

    // A control character has no literal JSON spelling at all, so passing one
    // through would produce a document the other side's parser rejects outright.
    #[test]
    fn control_characters_are_escaped_rather_than_passed_through() {
        assert_eq!(escape("a\nb\tc\r"), r"a\nb\tc\r");
        assert_eq!(escape("\u{1}"), r"\u0001");
    }
}
