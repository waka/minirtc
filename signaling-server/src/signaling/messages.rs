extern crate serde_derive;
extern crate serde_json;

use serde_json::json;

pub fn make_reply(text: String) -> String {
    let json: serde_json::Value = serde_json::from_str(&text).unwrap();
    let t = json["type"].as_str().unwrap();
    let reply = match t {
        "register" => register(json),
        "offer" => offer(json),
        "answer" => answer(json),
        "candidate" => candidate(json),
        "close" => close(json),
        _ => reject(json)
    };
    reply.to_string()
}

fn register(_json: serde_json::Value) -> serde_json::Value {
    json!({"type": "accept"})
}

fn reject(_json: serde_json::Value) -> serde_json::Value {
    json!({"type": "reject"})
}

fn close(_json: serde_json::Value) -> serde_json::Value {
    json!({"type": "close"})
}

fn offer(json: serde_json::Value) -> serde_json::Value {
    json!({"type": "offer", "sdp": json["sdp"]})
}

fn answer(json: serde_json::Value) -> serde_json::Value {
    json!({"type": "answer", "sdp": json["sdp"]})
}

fn candidate(json: serde_json::Value) -> serde_json::Value {
    json!({"type": "candidate", "ice": json["ice"]})
}
