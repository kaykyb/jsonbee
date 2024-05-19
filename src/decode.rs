use serde_json::{json, Value};

pub fn bencode_to_json(input: &str) -> Result<Value, String> {
    let mut chars = input.chars().peekable();
    decode_value(&mut chars)
}

fn decode_value(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Value, String> {
    match chars.peek() {
        Some(&'i') => decode_integer(chars),
        Some(&'l') => decode_list(chars),
        Some(&'d') => decode_dict(chars),
        Some(&('0'..='9')) => decode_string(chars),
        Some(_) => Err("Invalid Bencode format".to_string()),
        None => Err("Unexpected end of input".to_string()),
    }
}

fn decode_integer(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Value, String> {
    let mut num_str = String::new();
    chars.next(); // Skip 'i'

    while let Some(&ch) = chars.peek() {
        if ch == 'e' {
            chars.next(); // Skip 'e'
            break;
        }
        num_str.push(ch);
        chars.next();
    }

    num_str
        .parse::<i64>()
        .map(Value::from)
        .map_err(|_| "Invalid integer".to_string())
}

fn decode_string(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Value, String> {
    let mut length_str = String::new();

    while let Some(&ch) = chars.peek() {
        if ch == ':' {
            chars.next(); // Skip ':'
            break;
        }
        length_str.push(ch);
        chars.next();
    }

    let length = length_str
        .parse::<usize>()
        .map_err(|_| "Invalid string length".to_string())?;
    let mut string = String::with_capacity(length);
    for _ in 0..length {
        string.push(chars.next().ok_or("Unexpected end of string")?);
    }
    Ok(json!(string))
}

fn decode_list(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Value, String> {
    chars.next(); // Skip 'l'
    let mut list = vec![];

    while let Some(&ch) = chars.peek() {
        if ch == 'e' {
            chars.next(); // Skip 'e'
            break;
        }
        list.push(decode_value(chars)?);
    }
    Ok(json!(list))
}

fn decode_dict(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Value, String> {
    chars.next(); // Skip 'd'
    let mut dict = serde_json::Map::new();

    while let Some(&ch) = chars.peek() {
        if ch == 'e' {
            chars.next(); // Skip 'e'
            break;
        }
        let key = decode_string(chars)?.as_str().unwrap().to_string();
        let value = decode_value(chars)?;
        dict.insert(key, value);
    }
    Ok(json!(dict))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_integer() {
        let input = "i42e";
        let expected = json!(42);
        assert_eq!(bencode_to_json(input).unwrap(), expected);
    }

    #[test]
    fn test_decode_negative_integer() {
        let input = "i-42e";
        let expected = json!(-42);
        assert_eq!(bencode_to_json(input).unwrap(), expected);
    }

    #[test]
    fn test_decode_string() {
        let input = "4:spam";
        let expected = json!("spam");
        assert_eq!(bencode_to_json(input).unwrap(), expected);
    }

    #[test]
    fn test_decode_empty_string() {
        let input = "0:";
        let expected = json!("");
        assert_eq!(bencode_to_json(input).unwrap(), expected);
    }

    #[test]
    fn test_decode_list() {
        let input = "li42ei-42e4:spame";
        let expected = json!([42, -42, "spam"]);
        assert_eq!(bencode_to_json(input).unwrap(), expected);
    }

    #[test]
    fn test_decode_empty_list() {
        let input = "le";
        let expected = json!([]);
        assert_eq!(bencode_to_json(input).unwrap(), expected);
    }

    #[test]
    fn test_decode_dict() {
        let input = "d3:cow3:moo4:spam4:eggse";
        let expected = json!({"cow": "moo", "spam": "eggs"});
        assert_eq!(bencode_to_json(input).unwrap(), expected);
    }

    #[test]
    fn test_decode_empty_dict() {
        let input = "de";
        let expected = json!({});
        assert_eq!(bencode_to_json(input).unwrap(), expected);
    }

    #[test]
    fn test_decode_complex_structure() {
        let input = "d4:spamli42ee3:cowd4:food4:milkeee";
        let expected = json!({"spam": [42], "cow": {"food": "milk"}});
        assert_eq!(bencode_to_json(input).unwrap(), expected);
    }
}
