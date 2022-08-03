use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MyObj {
    name: String,
    number: i32,
}
