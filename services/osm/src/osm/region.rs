use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Region {
    Node {
        name: String,
        path: String,
        regions: Vec<Region>,
    },
    Leaf {
        name: String,
        file: String,
    },
}