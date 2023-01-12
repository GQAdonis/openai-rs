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
pub struct CreateCompletionResponseChoicesInnerLogprobs {
    #[serde(rename = "tokens", skip_serializing_if = "Option::is_none")]
    pub tokens: Option<Vec<String>>,
    #[serde(rename = "token_logprobs", skip_serializing_if = "Option::is_none")]
    pub token_logprobs: Option<Vec<f32>>,
    #[serde(rename = "top_logprobs", skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<Vec<serde_json::Value>>,
    #[serde(rename = "text_offset", skip_serializing_if = "Option::is_none")]
    pub text_offset: Option<Vec<i32>>,
}

impl CreateCompletionResponseChoicesInnerLogprobs {
    pub fn new() -> CreateCompletionResponseChoicesInnerLogprobs {
        CreateCompletionResponseChoicesInnerLogprobs {
            tokens: None,
            token_logprobs: None,
            top_logprobs: None,
            text_offset: None,
        }
    }
}

