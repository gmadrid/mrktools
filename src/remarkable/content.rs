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

/*
{
    "coverPageNumber": 0,
    "dummyDocument": false,
    "extraMetadata": {},
    "fileType": "pdf",
    "fontName": "",
    "lineHeight": -1,
    "margins": 100,
    "orientation": "portrait",
    "pageCount": 1,
    "pages": [
        "dfda01c2-43b2-4ca4-be20-80a6d6c7b183"
    ],
    "textAlignment": "left",
    "textScale": 1,
    "transform": {
        "m11": 1,
        "m12": 0,
        "m13": 0,
        "m21": 0,
        "m22": 1,
        "m23": 0,
        "m31": 0,
        "m32": 0,
        "m33": 1
    }
}
*/
