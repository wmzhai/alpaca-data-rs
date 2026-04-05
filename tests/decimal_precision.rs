use std::str::FromStr;

use alpaca_data::Decimal;

#[test]
fn crate_root_decimal_reexport_preserves_trailing_zero_scale() {
    let value = Decimal::from_str("175.10").expect("decimal literal should parse");

    assert_eq!(value.to_string(), "175.10");
}

#[test]
fn serde_json_decimal_preserves_trailing_zero_scale() {
    let value: Decimal = serde_json::from_str("175.10")
        .expect("decimal payload should deserialize with preserved scale");

    assert_eq!(value.to_string(), "175.10");
}

#[test]
fn serde_json_decimal_preserves_small_fraction_scale() {
    let value: Decimal = serde_json::from_str("0.000073")
        .expect("small fraction should deserialize with preserved scale");

    assert_eq!(value.to_string(), "0.000073");
}

#[test]
fn serde_json_decimal_preserves_signed_scale() {
    let value: Decimal = serde_json::from_str("-0.043")
        .expect("signed decimal should deserialize with preserved scale");

    assert_eq!(value.to_string(), "-0.043");
}
