use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct Body {
    parent: String,
    properties: Properties,
}

#[derive(Serialize, Deserialize)]
pub struct Properties {
    title: Title,
}

#[derive(Serialize, Deserialize)]
pub struct Title {
    title: Vec<Text>,
}

#[derive(Serialize, Deserialize)]
pub struct Text {
    content: String,
}

impl Body {
    pub fn new(database_id: &'static str) -> Self {
        Body {
            parent: String::from(database_id),
            properties: Properties {
                title: Title {
                    title: vec![Text {
                        content: String::from("Hello there!"),
                    }],
                },
            },
        }
    }
}
