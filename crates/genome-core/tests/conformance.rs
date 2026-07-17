//! Serde and validation conformance: every public enum variant round-trips
//! through JSON, the tag names are pinned, `Feature::key` renders the canonical
//! axis names, an unknown indicator is a build error, and `GenomeSpec::validate`
//! rejects an empty feature or symbol set.

use std::collections::BTreeMap;

use genome_core::{build, Candle, Feature, GenomeSpec, Metric, Normalize, PriceField};

fn json_round_trip<T>(value: &T)
where
    T: serde::Serialize + serde::de::DeserializeOwned + PartialEq + std::fmt::Debug,
{
    let text = serde_json::to_string(value).expect("serialize");
    let back: T = serde_json::from_str(&text).expect("deserialize");
    assert_eq!(&back, value, "JSON round-trip must be stable");
}

#[test]
fn feature_variants_round_trip() {
    json_round_trip(&Feature::Price {
        field: PriceField::Close,
    });
    json_round_trip(&Feature::Indicator {
        name: "Rsi".into(),
        params: vec![14.0],
        field: None,
    });
    json_round_trip(&Feature::Indicator {
        name: "Macd".into(),
        params: vec![12.0, 26.0, 9.0],
        field: Some("hist".into()),
    });
    // The tag is snake_case on `kind`.
    assert_eq!(
        serde_json::to_string(&Feature::Price {
            field: PriceField::Volume
        })
        .unwrap(),
        "{\"kind\":\"price\",\"field\":\"volume\"}"
    );
}

#[test]
fn price_field_variants_round_trip() {
    for f in [
        PriceField::Open,
        PriceField::High,
        PriceField::Low,
        PriceField::Close,
        PriceField::Volume,
    ] {
        json_round_trip(&f);
    }
    assert_eq!(
        serde_json::to_string(&PriceField::High).unwrap(),
        "\"high\""
    );
}

#[test]
fn normalize_and_metric_variants_round_trip() {
    for n in [Normalize::ZScore, Normalize::MinMax] {
        json_round_trip(&n);
    }
    for m in [Metric::Cosine, Metric::Euclid] {
        json_round_trip(&m);
    }
    assert_eq!(
        serde_json::to_string(&Normalize::MinMax).unwrap(),
        "\"min_max\""
    );
    assert_eq!(
        serde_json::to_string(&Metric::Euclid).unwrap(),
        "\"euclid\""
    );
}

#[test]
fn feature_keys_are_canonical() {
    assert_eq!(
        Feature::Price {
            field: PriceField::Close
        }
        .key(),
        "price.close"
    );
    assert_eq!(
        Feature::Indicator {
            name: "Rsi".into(),
            params: vec![14.0],
            field: None,
        }
        .key(),
        "Rsi(14)"
    );
    assert_eq!(
        Feature::Indicator {
            name: "Macd".into(),
            params: vec![12.0, 26.0, 9.0],
            field: Some("hist".into()),
        }
        .key(),
        "Macd(12,26,9).hist"
    );
}

#[test]
fn spec_round_trips_through_json() {
    let spec: GenomeSpec = serde_json::from_str(
        r#"{"features":[{"kind":"price","field":"close"},
            {"kind":"indicator","name":"Rsi","params":[14]}],
            "symbols":["AAA","BBB"],"normalize":"z_score","metric":"cosine","seed":7}"#,
    )
    .unwrap();
    json_round_trip(&spec);
    assert_eq!(spec.features.len(), 2);
    assert_eq!(spec.metric, Metric::Cosine);
}

#[test]
fn empty_features_is_a_spec_error() {
    // `from_json` parses then validates; an empty feature set must be rejected.
    assert!(
        GenomeSpec::from_json(
            r#"{"features":[],"symbols":["AAA"],"normalize":"z_score","metric":"euclid"}"#,
        )
        .is_err(),
        "a spec with no feature axes must be rejected"
    );
}

#[test]
fn empty_symbols_is_a_spec_error() {
    assert!(
        GenomeSpec::from_json(
            r#"{"features":[{"kind":"price","field":"close"}],"symbols":[],
            "normalize":"z_score","metric":"euclid"}"#,
        )
        .is_err(),
        "a spec with no symbols must be rejected"
    );
}

#[test]
fn unknown_indicator_is_a_build_error() {
    let spec: GenomeSpec = serde_json::from_str(
        r#"{"features":[{"kind":"indicator","name":"NotAnIndicator","params":[1]}],
            "symbols":["AAA"],"normalize":"z_score","metric":"euclid"}"#,
    )
    .unwrap();
    let mut data: BTreeMap<String, Vec<Candle>> = BTreeMap::new();
    data.insert(
        "AAA".into(),
        vec![Candle {
            time: 0,
            open: 1.0,
            high: 1.0,
            low: 1.0,
            close: 1.0,
            volume: 0.0,
        }],
    );
    assert!(
        build(&data, &spec).is_err(),
        "an unknown indicator name must be a build error"
    );
}
