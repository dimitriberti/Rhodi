use crate::error::{Result, RhodiError};
use jsonpath_rust::JsonPathFinder;
use regex::Regex;
use serde_json::Value;

pub trait Extractor {
    fn extract(&self, source: &[u8], selector: &str) -> Result<String>;
}

pub struct RegexExtractor;

impl Extractor for RegexExtractor {
    fn extract(&self, source: &[u8], selector: &str) -> Result<String> {
        let text = String::from_utf8_lossy(source);
        let re = Regex::new(selector)
            .map_err(|e| RhodiError::Extraction(format!("Invalid regex '{}': {}", selector, e)))?;

        if let Some(caps) = re.captures(&text) {
            // If there's a capture group, return the first one, otherwise the whole match
            let val = caps.get(1).or_else(|| caps.get(0)).unwrap().as_str();
            Ok(val.to_string())
        } else {
            Err(RhodiError::Extraction(format!(
                "Regex '{}' found no matches",
                selector
            )))
        }
    }
}

pub struct JsonPathExtractor;

impl Extractor for JsonPathExtractor {
    fn extract(&self, source: &[u8], selector: &str) -> Result<String> {
        let json: Value = serde_json::from_slice(source)
            .map_err(|e| RhodiError::Extraction(format!("Invalid JSON for extraction: {}", e)))?;

        let finder = JsonPathFinder::from_str(&json.to_string(), selector).map_err(|e| {
            RhodiError::Extraction(format!("Invalid JSONPath '{}': {}", selector, e))
        })?;

        let found = finder.find();

        if found.is_null() || (found.is_array() && found.as_array().unwrap().is_empty()) {
            return Err(RhodiError::Extraction(format!(
                "JSONPath '{}' found no matches",
                selector
            )));
        }

        // If it's an array with one element, return that element as string, otherwise the whole thing
        if let Some(arr) = found.as_array() && arr.len() == 1 {
            return Ok(value_to_string(&arr[0]));
        }

        Ok(value_to_string(&found))
    }
}

fn value_to_string(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        _ => v.to_string(),
    }
}

pub fn get_extractor(method: &str) -> Result<Box<dyn Extractor>> {
    match method.to_lowercase().as_str() {
        "regex" => Ok(Box::new(RegexExtractor)),
        "jsonpath" => Ok(Box::new(JsonPathExtractor)),
        _ => Err(RhodiError::Extraction(format!(
            "Unknown extraction method: {}",
            method
        ))),
    }
}
