use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;

pub fn assert_presence(input: &Value, output: &Value, path: &str) {
    match input {
        Value::Object(object) => {
            let actual = output
                .as_object()
                .unwrap_or_else(|| panic!("object changed at {path}"));
            assert_eq!(
                object.len(),
                actual.len(),
                "object presence changed at {path}"
            );
            for (key, value) in object {
                let next = format!("{path}/{key}");
                assert_presence(
                    value,
                    actual
                        .get(key)
                        .unwrap_or_else(|| panic!("lost field {next}")),
                    &next,
                );
            }
        }
        Value::Array(array) => {
            let actual = output
                .as_array()
                .unwrap_or_else(|| panic!("array changed at {path}"));
            assert_eq!(array.len(), actual.len(), "array length changed at {path}");
            for (index, (a, b)) in array.iter().zip(actual).enumerate() {
                assert_presence(a, b, &format!("{path}/{index}"));
            }
        }
        Value::Null => assert!(output.is_null(), "null changed at {path}"),
        _ => assert!(!output.is_null(), "value became null at {path}"),
    }
}

fn null_mutations(value: &Value, path: &str, output: &mut Vec<(String, Value)>) {
    match value {
        Value::Object(object) => {
            for (key, child) in object {
                let next = format!("{path}/{key}");
                let mut null = value.clone();
                null[key] = Value::Null;
                output.push((next.clone(), null));
                let mut nested = Vec::new();
                null_mutations(child, &next, &mut nested);
                for (path, mutation) in nested {
                    let mut copy = value.clone();
                    copy[key] = mutation;
                    output.push((path, copy));
                }
            }
        }
        Value::Array(array) => {
            for (index, child) in array.iter().enumerate() {
                let next = format!("{path}/{index}");
                let mut null = value.clone();
                null[index] = Value::Null;
                output.push((next.clone(), null));
                let mut nested = Vec::new();
                null_mutations(child, &next, &mut nested);
                for (path, mutation) in nested {
                    let mut copy = value.clone();
                    copy[index] = mutation;
                    output.push((path, copy));
                }
            }
        }
        _ => {}
    }
}

pub fn assert_nulls<T: DeserializeOwned + Serialize>(name: &str, fixture: &Value) -> usize {
    let mut cases = Vec::new();
    null_mutations(fixture, name, &mut cases);
    for (path, value) in &cases {
        let valid = kalshi_client::typed::validate(name, value).is_ok();
        let result = serde_json::from_value::<T>(value.clone());
        if valid {
            let model = result.unwrap_or_else(|_| panic!("valid null rejected at {path}"));
            assert_presence(value, &serde_json::to_value(model).unwrap(), path);
        } else {
            assert!(result.is_err(), "invalid null accepted at {path}");
        }
    }
    cases.len()
}
