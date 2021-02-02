use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    #[serde(rename = "coverPageNumber")]
    cover_page_number: u32,

    #[serde(rename = "dummyDocument")]
    dummy_document: bool,

    #[serde(rename = "extraMetadata")]
    extra_metadata: HashMap<String, String>,

    #[serde(rename = "fileType")]
    file_type: String,

    #[serde(rename = "fontName")]
    font_name: String,

    #[serde(rename = "lineHeight")]
    line_height: i32,

    margins: u32,
    orientation: String,

    #[serde(rename = "pageCount")]
    page_count: usize,

    pages: Vec<String>,

    #[serde(rename = "textAlignment")]
    text_alignment: String,

    #[serde(rename = "textScale")]
    text_scale: u32,

    transform: HashMap<String, i32>,
}

impl Content {
    pub fn add_page(&mut self, s: impl Into<String>) {
        self.pages.push(s.into());
        self.page_count = self.pages.len();
    }
}

impl Default for Content {
    fn default() -> Self {
        let mut transform = HashMap::default();
        transform.insert("m11".into(), 1);
        transform.insert("m12".into(), 0);
        transform.insert("m13".into(), 0);
        transform.insert("m21".into(), 0);
        transform.insert("m22".into(), 1);
        transform.insert("m23".into(), 0);
        transform.insert("m31".into(), 0);
        transform.insert("m32".into(), 0);
        transform.insert("m33".into(), 1);

        Content {
            cover_page_number: 0,
            dummy_document: false,
            extra_metadata: Default::default(),
            file_type: "pdf".into(),
            font_name: "".into(),
            line_height: -1,
            margins: 100,
            orientation: "portrait".into(),
            page_count: 0,
            pages: Default::default(),
            text_alignment: "left".into(),
            text_scale: 1,
            transform,
        }
    }
}
