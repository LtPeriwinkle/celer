use std::collections::BTreeMap;

use serde_json::{json, Map, Value};

use crate::json::Coerce;

use super::CompilerError;

type DesugarLine = (String, Map<String, Value>);
type DesugarLineError = (String, CompilerError);

/// Desugar a line as a json blob
///
/// Valid line formats:
/// - object with one key, and value is an object
/// - string (desugared to `{[value]: {}}`)
/// - null (desugared to `{"": {}}`)
/// - boolean, number (desugared to string representation)
pub async fn desugar_line(value: Value) -> Result<DesugarLine, DesugarLineError> {
    let text = value.coerce_to_string();
    match value {
        Value::Array(_) => Err((text, CompilerError::ArrayCannotBeLine)),
        Value::Object(obj) => {
            let mut iter = obj.into_iter();
            let (key, obj) = match iter.next() {
                None => {
                    return Err((text, CompilerError::EmptyObjectCannotBeLine));
                }
                Some(first) => first,
            };
            if iter.next().is_some() {
                return Err((text, CompilerError::TooManyKeysInObjectLine));
            }
            let properties = match obj {
                Value::Object(map) => map,
                _ => {
                    return Err((text, CompilerError::LinePropertiesMustBeObject));
                }
            };
            Ok((key, properties))
        }
        _ => Ok((text, Map::new())),
    }
}

/// Desugar properties on a line
///
/// Some properties like `coord` are simply short-hand for other properties.
/// They are converted to their long-hand form here.
pub async fn desugar_properties(properties: &mut BTreeMap<String, Value>) {
    if let Some(value) = properties.remove("coord") {
        properties.insert("movements".to_string(), json!([value]));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_line_primitive() {
        assert_eq!(
            desugar_line(json!(null)).await,
            Ok(("".to_string(), Map::new()))
        );
        assert_eq!(
            desugar_line(json!(true)).await,
            Ok(("true".to_string(), Map::new()))
        );
        assert_eq!(
            desugar_line(json!(false)).await,
            Ok(("false".to_string(), Map::new()))
        );
        assert_eq!(
            desugar_line(json!("")).await,
            Ok(("".to_string(), Map::new()))
        );
        assert_eq!(
            desugar_line(json!("hello world")).await,
            Ok(("hello world".to_string(), Map::new()))
        );
    }

    #[tokio::test]
    async fn test_line_array() {
        assert_eq!(
            desugar_line(json!([])).await,
            Err((
                "[object array]".to_string(),
                CompilerError::ArrayCannotBeLine
            ))
        );
    }

    #[tokio::test]
    async fn test_line_object_invalid() {
        let str = "[object object]";
        assert_eq!(
            desugar_line(json!({})).await,
            Err((str.to_string(), CompilerError::EmptyObjectCannotBeLine))
        );
        assert_eq!(
            desugar_line(json!({"one":"two", "three":"four" })).await,
            Err((str.to_string(), CompilerError::TooManyKeysInObjectLine))
        );
        assert_eq!(
            desugar_line(json!({"one": []})).await,
            Err((str.to_string(), CompilerError::LinePropertiesMustBeObject))
        );
    }

    #[tokio::test]
    async fn test_line_object_valid() {
        assert_eq!(
            desugar_line(json!({"one": {
                "two": "three"
            }}))
            .await,
            Ok((
                "one".to_string(),
                [{ ("two".to_string(), json!("three")) }]
                    .into_iter()
                    .collect()
            ))
        );
    }

    #[tokio::test]
    async fn test_properties_empty() {
        let mut properties = BTreeMap::new();
        desugar_properties(&mut properties).await;
        assert_eq!(properties, BTreeMap::new());
    }

    #[tokio::test]
    async fn test_properties_coord() {
        let mut properties = BTreeMap::new();
        properties.insert("coord".to_string(), json!([1, 2]));
        desugar_properties(&mut properties).await;
        assert!(properties.get("coord").is_none());
        assert_eq!(properties.get("movements").unwrap(), &json!([[1, 2]]));
    }
}