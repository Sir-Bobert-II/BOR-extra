use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serenity::{builder::CreateApplicationCommand, model::prelude::command::CommandOptionType};
use thiserror::Error;

lazy_static! {
    pub static ref HELP: String = {
        help::HelpMessage::new()
            .name("image")
            .description("Request images from the internet")
            .add_subcommand(
                help::HelpMessage::new()
                    .name("random_dog")
                    .description("Request a random dog image")
                    .clone(),
            )
            .add_subcommand(
                help::HelpMessage::new()
                    .name("random_cat")
                    .description("Request a random cat image")
                    .clone(),
            )
            .to_string()
    };
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("image")
        .dm_permission(true)
        .description("Request images from the internet")
        .create_option(|option| {
            option
                .name("random_dog")
                .kind(CommandOptionType::SubCommand)
                .description("Request a random dog image")
        })
        .create_option(|option| {
            option
                .name("random_cat")
                .kind(CommandOptionType::SubCommand)
                .description("Request a random cat image")
        })
}

#[derive(Debug, Error, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ImageError {
    #[error("JSONParseError: {message}")]
    JsonParse { message: String },

    #[error("RequestError: {message}")]
    Request { message: String },

    #[error(
        "CooldownError: Please wait {duration} second(s) before trying to use this image command again."
    )]
    Cooldown { duration: String },
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct Image {
    link: String,
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.link)
    }
}

impl From<DogImage> for Image {
    fn from(value: DogImage) -> Self {
        Self {
            link: value.message,
        }
    }
}

impl From<CatImage> for Image {
    fn from(value: CatImage) -> Self {
        Self { link: value.url }
    }
}

/// A random image of a dog
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatImage {
    pub id: String,
    pub url: String,
    pub width: i64,
    pub height: i64,
}

impl CatImage {
    pub fn random() -> Result<Self, ImageError> {
        pub type CatImages = Vec<CatImage>;
        // Request URL
        let url = "https://api.thecatapi.com/v1/images/search".to_string();

        // Get the response
        if let Ok(resp) = match reqwest::blocking::get(url) {
            Ok(x) => x,
            Err(e) => {
                return Err(ImageError::Request {
                    message: format!("{e}"),
                })
            }
        }
        .json::<CatImages>()
        {
            Ok(match resp.get(0) {
                Some(x) => x.clone(),
                _ => {
                    return Err(ImageError::JsonParse {
                        message: "The returned json was empty!".to_string(),
                    })
                }
            })
        } else {
            Err(ImageError::JsonParse {
                message: "Invalid JSON content".to_string(),
            })
        }
    }
}

/// A random image of a dog
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DogImage {
    status: String,
    message: String,
}

impl DogImage {
    pub fn random() -> Result<Self, ImageError> {
        // Request URL
        let url = "https://dog.ceo/api/breeds/image/random".to_string();

        // Get the response
        if let Ok(resp) = match reqwest::blocking::get(url) {
            Ok(x) => x,
            Err(e) => {
                return Err(ImageError::Request {
                    message: format!("{e}"),
                })
            }
        }
        .json::<Self>()
        {
            Ok(resp)
        } else {
            Err(ImageError::JsonParse {
                message: "Invalid JSON content".to_string(),
            })
        }
    }
}
