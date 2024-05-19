use serde_json::Value;
use std::collections::BTreeMap;

pub fn json_to_bencode(input: &str) -> Result<String, serde_json::Error> {
    match serde_json::from_str(input) {
        Ok(json_value) => Ok(encode_bencode(&json_value)),
        Err(e) => Err(e),
    }
}

fn encode_bencode(value: &Value) -> String {
    match value {
        Value::Number(num) => format!("i{}e", num),
        Value::String(s) => format!("{}:{}", s.len(), s),
        Value::Array(vec) => {
            let mut encoded = String::from("l");
            for item in vec {
                encoded.push_str(&encode_bencode(item));
            }
            encoded.push('e');
            encoded
        }
        Value::Object(map) => {
            let mut encoded = String::from("d");
            let sorted_map = map.iter().collect::<BTreeMap<_, _>>();
            for (key, val) in sorted_map {
                encoded.push_str(&format!("{}:{}", key.len(), key));
                encoded.push_str(&encode_bencode(val));
            }
            encoded.push('e');
            encoded
        }
        _ => String::new(), // Boolean or others, typically not encoded in Bencode
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_integer() {
        let json = "42";
        let expected = "i42e";
        assert_eq!(json_to_bencode(json).unwrap(), expected);
    }

    #[test]
    fn test_encode_string() {
        let json = "\"hello\"";
        let expected = "5:hello";
        assert_eq!(json_to_bencode(json).unwrap(), expected);
    }

    #[test]
    fn test_encode_list() {
        let json = "[1, 2, 3]";
        let expected = "li1ei2ei3ee";
        assert_eq!(json_to_bencode(json).unwrap(), expected);
    }

    #[test]
    fn test_encode_dict() {
        let json = r#"{"cat":"mew","dog":"woof"}"#;
        let expected = "d3:cat3:mew3:dog4:woofe";
        assert_eq!(json_to_bencode(json).unwrap(), expected);
    }
}
