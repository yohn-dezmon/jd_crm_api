use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

#[derive(Type, Serialize, Deserialize)]
#[sqlx(type_name = "media_type", rename_all = "lowercase")]
pub enum MediaType {
    Audio,
    Video,
    Web,
    Book,
    ScientificArticle,
}

#[derive(Type, Serialize, Deserialize)]
#[sqlx(type_name = "image_type", rename_all = "lowercase")]
pub enum ImageType {
    PDF,
    PNG,
    TIFF,
    JPEG,
    GIF,
}

#[derive(Deserialize, FromRow)]
pub struct CreateSource {
    pub name: String,
    pub url: Option<String>,
    pub author: Option<String>,
    pub author_url: Option<String>,
    pub media_type: Option<MediaType>,
    pub image_url: Option<String>,
    pub image_type: Option<ImageType>,
    pub ai_generated: Option<bool>,
    pub related_terms: Option<Vec<String>>,
    pub related_topics: Option<Vec<String>>,
    pub related_sources: Option<Vec<String>>,
}

/*
I need to import the above into handler_utils as well as sources.rs
 */
