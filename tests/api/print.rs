// Copyright 2022 Martin Pool

//! Test printing values

use proptest::prelude::*;

use rsj::array::Array;
use rsj::atom::Atom;

proptest! {
    #[test]
    fn number_array_print_length_limited(a: Vec<f64>) {
        let lim = 80;
        let arr = Array::from_iter(a.iter().cloned().map(Atom::from));
        let p = format!("{:.*}", lim, arr);
        dbg!(&p);
        // A single number alone is always printed, even if it's very long.
        if a.len() == 1 {
            assert!(!p.ends_with("..."));
            assert!(!p.contains(' '));
        }
        else {
            if let Some(b) = p.strip_suffix(" ...") {
                // If we only managed to print one number, it's OK if it's more
                // than lim. Otherwise, the whole line should be under lim.
                if b.contains(' ') {
                    assert!(b.len() <= lim);
                }
            } else {
                assert!(p.len() <= lim, "with no elipsis the line should be shorter than the lim");
            }
        }
    }
}
