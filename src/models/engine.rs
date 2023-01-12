/*
 * OpenAI API
 *
 * APIs for sampling from and fine-tuning language models
 *
 * The version of the OpenAPI document: 1.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Engine {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "object")]
    pub object: String,
    #[serde(rename = "created", deserialize_with = "Option::deserialize")]
    pub created: Option<i32>,
    #[serde(rename = "ready")]
    pub ready: bool,
}

impl Engine {
    pub fn new(id: String, object: String, created: Option<i32>, ready: bool) -> Engine {
        Engine {
            id,
            object,
            created,
            ready,
        }
    }
}

