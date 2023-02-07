use serde_derive::{Deserialize, Serialize};

// https://github.com/lukePeavey/quotable#get-random-quote
#[derive(Error,Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum QuoteError
{
    #[error("JSONParseError: couldn't parse json: {message}")]
    JsonParse {message: String},

    #[default]
    #[error("Error: An unknown error occurred.")]
    GeneralError,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RandomQuote {
    #[serde(rename = "_id")]
    pub id: String,
    pub content: String,
    pub author: String,
    pub tags: Vec<String>,
    pub author_slug: String,
    pub length: i64,
    pub date_added: String,
    pub date_modified: String,
}