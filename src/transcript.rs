// Copyright 2021 Martin Pool

//! Handle J transcript files.

use crate::error::Result;
use crate::eval::Session;

pub fn rerun(session: &mut Session, ts: &str) -> Result<String> {
    let mut out = String::new();
    for l in ts.lines() {
        if let Some(s) = l.strip_prefix("   ") {
            assert!(!s.starts_with(' ')); // no extra spaces: does not actually need to be true but might catch indentation bugs
            out.push_str(l);
            out.push('\n');
            let output = session.eval_text(s);
            assert!(!output.ends_with('\n'));
            out.push_str(&output);
            out.push('\n');
        }
    }
    Ok(out)
}
