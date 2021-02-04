use crate::imgtools::process_image;
use crate::remarkable::Connection;
use crate::remarkable::{create_bare_fs, Content, Metadata, METADATA_EXTENSION};
use crate::{Error, Result};
use argh::FromArgs;
use log::{error, info};
use printpdf::*;
use std::borrow::Cow;
use std::ffi::OsString;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use uuid::Uuid;

const DEFAULT_DEST_DIR: &str = "./rem";
const DPI: f64 = 300.0;

#[derive(FromArgs, Debug)]
/// convert images to Remarkable pdfs with a thumbnail
#[argh(subcommand, name = "ipdf")]
pub struct IPdfArgs {
    /// alpha value to be multiplied by the image, range [0-100].
    #[argh(option, short = 'a', default = "100")]
    alpha: u8,

    /// convert pdf to grayscale
    #[argh(switch, short = 'g')]
    to_gray: bool,

    /// file names to convert to Remarkable FDF files
    #[argh(positional)]
    file_names: Vec<String>,

    /// if present, generated files will be put in the specified folder on the Remarkable
    #[argh(option, short = 'p')]
    parent: Option<String>,

    /// directory for output files
    #[argh(option, short = 'o', default = "DEFAULT_DEST_DIR.to_string()")]
    dest_dir: String,
}

pub fn ipdf(conn: &mut Connection, opt: IPdfArgs) -> Result<()> {
    let should_print = opt.file_names.len() > 1;
    for file in opt.file_names {
        if should_print {
            let base_fn = PathBuf::from(&file)
                .file_name()
                .map(OsString::from)
                .unwrap_or_else(|| OsString::from(""));
            info!("Processing: {}", base_fn.to_string_lossy());
        }

        let parent_id = opt
            .parent
            .as_ref()
            .map(|p| conn.find_folder(p))
            .transpose()?;
        if let Err(err) = ipdf_func(file, opt.to_gray, opt.alpha, parent_id, &opt.dest_dir) {
            error!("{}", err);
        }
    }
    Ok(())
}

fn ipdf_func(
    img: impl AsRef<Path>,
    to_gray: bool,
    alpha: u8,
    parent: Option<impl AsRef<str>>,
    output_dir: impl AsRef<Path>,
) -> Result<()> {
    if alpha > 100 {
        return Err(Error::AlphaRangeError(alpha));
    }

    if !output_dir.as_ref().exists() {
        std::fs::create_dir(&output_dir)?;
    }
    let uu = new_uuid();
    let base = create_bare_fs(&uu, &output_dir)?;

    let image = open_image(img.as_ref())?;
    let processed_image = process_image(&image, to_gray, alpha)?;

    let pdf = create_pdf(&img.as_ref().to_string_lossy(), &processed_image);
    let outfile = base.with_extension("pdf");
    pdf.save(&mut BufWriter::new(File::create(outfile)?))?;

    let content_file = File::create(base.with_extension("content"))?;
    let mut content = Content::default();
    let page_uuid = new_uuid();
    content.add_page(&page_uuid);
    serde_json::to_writer(content_file, &content)?;

    create_metadata_file(
        &img.as_ref()
            .file_name()
            .map(|f| f.to_string_lossy())
            .unwrap_or(Cow::Borrowed("<null>")),
        &base,
        parent,
    )?;
    create_pagedata_file(&base)?;

    let small_image = resize_image(&image, 362, 512);
    write_thumbnail(&small_image, base, &page_uuid)?;

    Ok(())
}

fn new_uuid() -> String {
    let uu = Uuid::new_v4();
    uu.to_hyphenated()
        .encode_lower(&mut Uuid::encode_buffer())
        .to_string()
}

fn open_image(path: &Path) -> Result<image::DynamicImage> {
    Ok(printpdf::image::io::Reader::open(path)?.decode()?)
}

fn create_pdf(doc_name: &str, img_view: &image::DynamicImage) -> PdfDocumentReference {
    let pdf_image = Image::from_dynamic_image(img_view);
    let (doc, page, layer) = PdfDocument::new(
        doc_name,
        pdf_image.image.width.into_pt(DPI).into(),
        pdf_image.image.height.into_pt(DPI).into(),
        "Layer 1",
    );

    let current_layer = doc.get_page(page).get_layer(layer);
    pdf_image.add_to_layer(current_layer, None, None, None, None, None, Some(DPI));

    doc
}

fn resize_image(image: &image::DynamicImage, width: u32, height: u32) -> image::DynamicImage {
    image.resize(width, height, image::imageops::FilterType::Nearest)
}
fn write_thumbnail(
    image: &image::DynamicImage,
    base: impl AsRef<Path>,
    uuid: impl AsRef<str>,
) -> Result<()> {
    let thumb_dir = base.as_ref().with_extension("thumbnails");
    let image_name = thumb_dir.join(uuid.as_ref()).with_extension("jpg");
    if !thumb_dir.exists() {
        std::fs::create_dir(thumb_dir)?;
    }
    let mut image_file = File::create(image_name)?;
    image.write_to(&mut image_file, image::ImageOutputFormat::Jpeg(50))?;

    Ok(())
}

fn create_metadata_file(
    file_name: impl AsRef<str>,
    base: impl AsRef<Path>,
    parent: Option<impl AsRef<str>>,
) -> Result<()> {
    let metadata_file = File::create(base.as_ref().with_extension(METADATA_EXTENSION))?;
    let metadata = Metadata::with_name_and_parent(
        file_name,
        parent.as_ref().map(|p| p.as_ref()).unwrap_or(""),
    );

    serde_json::to_writer(metadata_file, &metadata)?;
    Ok(())
}

fn create_pagedata_file(base: impl AsRef<Path>) -> Result<()> {
    // Right now, there is only ever one file.
    let mut pagedata_file = File::create(base.as_ref().with_extension("pagedata"))?;
    writeln!(pagedata_file, "Blank")?;

    Ok(())
}
