use std::str::FromStr;

use alpaca_data::{Decimal, corporate_actions};

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

#[test]
fn serde_json_decimal_serializes_with_preserved_scale() {
    let value = Decimal::from_str("175.10").expect("decimal literal should parse");

    let serialized =
        serde_json::to_string(&value).expect("decimal value should serialize to JSON");

    assert_eq!(serialized, "\"175.10\"");
}

#[test]
fn unknown_corporate_action_values_preserve_exact_numeric_lexemes() {
    let response: corporate_actions::ListResponse = serde_json::from_str(
        r#"{
            "corporate_actions": {
                "mystery_bucket": [
                    {
                        "id": "mystery-1",
                        "quoted_rate": 175.10,
                        "tiny_rate": 0.000073
                    }
                ]
            },
            "next_page_token": null
        }"#,
    )
    .expect("list response should deserialize");

    let action = &response.corporate_actions.other["mystery_bucket"][0];
    let quoted_rate = action
        .get("quoted_rate")
        .expect("unknown corporate action should contain quoted_rate");
    let tiny_rate = action
        .get("tiny_rate")
        .expect("unknown corporate action should contain tiny_rate");

    assert_eq!(quoted_rate.to_string(), "175.10");
    assert_eq!(tiny_rate.to_string(), "0.000073");
}
