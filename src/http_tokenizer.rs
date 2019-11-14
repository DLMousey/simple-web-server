use std::borrow::Cow;

// TODO: maybe add user_agent, content_length, connection in the future
pub struct HttpReq {
    pub method: String,
     host: String,
     content_type: String,
     body: String,
}

pub fn tokenize(req: Cow<'_, str>) -> HttpReq {

    // Initialize an instance of the HttpReq struct with some default values to keep the compiler happy.
    let mut t: HttpReq = HttpReq {
        method: "UNKNOWN".to_string(),
        host: "UNKNOWN".to_string(),
        content_type: "UNKNOWN".to_string(),
        body: "UNKNOWN".to_string()
    };

    for (index, line_val) in req.lines().enumerate() {
        // Because the method doesn't follow a Key: val structure, we'll count our blessings
        // that the spec defines it as the very first thing in the request and match it first.
        if index == 0 {
            t.method = get_key(line_val);
        }

        // All other values in the headers are following a Key: Val structure,
        // so we'll break these apart into a normalised key (snake_cased) and a value.
        // For each key we derive from the request headers, we'll run a match.
        // If the normalized_key is a match to any of the fields in the struct we'll set
        // the value of that field to the value we've derived from the request.
        let normalized_key = get_key(line_val)
            .to_lowercase()
            .replace("-", "_")
            .replace(":", "");

        let value = get_value(line_val);

        // @todo - Refactor into a less disgusting mess, couldn't get this working with `match`
        if normalized_key == String::from("host") {
            t.host = value;
        } else if normalized_key == String::from("content_type") {
            t.content_type = value;
        } else if normalized_key == String::from("body") {
            t.body = value;
        }
    }

    return t;
}

fn get_key(line: &str) -> String {
    return line.split_whitespace()
        .next()
        .unwrap_or("")
        .to_string();
}

fn get_value(line: &str) -> String {
    let mut split = line.split(": ");

    split.next(); // Move the iterator along once...
    return split.next().unwrap_or("").to_string(); // Move the iterator along again and return!
}