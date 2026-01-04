//! Heartbeat structs

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Heartbeat {
    filename: String,
    iat_sec: f64,
    editor: String,
    line: usize,
    column: usize,
    operating_system: String,
    user_agent: String,
    ext_metadata: HashMap<String, String>
}