use serde_json::Value;

pub fn p1(input: &str) -> i64 {
    let json: Value = serde_json::from_str(input).unwrap();
    extract_sum(&json, false)
}

pub fn p2(input: &str) -> i64 {
    let json: Value = serde_json::from_str(input).unwrap();
    extract_sum(&json, true)
}

fn extract_sum(json: &Value, skip_red: bool) -> i64 {
    match json {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(arr) => arr.iter().map(|v| extract_sum(v, skip_red)).sum(),
        Value::Object(obj) => {
            if skip_red && obj.values().any(|v| v == "red") {
                0
            } else {
                obj.values()
                    .fold(0, |acc, v| acc + extract_sum(v, skip_red))
            }
        }
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(p1("[1,2,3]"), 6);
        assert_eq!(p1("{\"a\":2,\"b\":4}"), 6);
        assert_eq!(p1("[[[3]]]"), 3);
        assert_eq!(p1("{\"a\":{\"b\":4},\"c\":-1}"), 3);
        assert_eq!(p1("[]"), 0);
        assert_eq!(p1("{}"), 0);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2("[1,2,3]"), 6);
        assert_eq!(p2("[1,{\"c\":\"red\",\"b\":2},3]"), 4);
        assert_eq!(p2("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}"), 0);
        assert_eq!(p2("[1,\"red\",5]"), 6);
    }
}
