// Copyright 2022 Martin Pool

use rsj::primitive::Primitive;

#[test]
fn primitive_debug_repr() {
    let add = Primitive::by_name(&"+").unwrap();
    assert_eq!(format!("{:?}", add), r#"Primitive { name: "+" }"#);
}
