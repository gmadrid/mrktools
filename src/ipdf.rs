use crate::imgtools::process_image;
use crate::remarkable::{create_bare_fs, Content, Metadata, METADATA_EXTENSION};
use crate::{Error, Result};
use printpdf::*;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

const DPI: f64 = 300.0;

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

fn create_metadata_file(file_name: impl AsRef<str>, base: impl AsRef<Path>) -> Result<()> {
    let metadata_file = File::create(base.as_ref().with_extension(METADATA_EXTENSION))?;
    let metadata = Metadata::with_visible_name(file_name);

    serde_json::to_writer(metadata_file, &metadata)?;
    Ok(())
}

fn create_pagedata_file(base: impl AsRef<Path>) -> Result<()> {
    // Right now, there is only ever one file.
    let mut pagedata_file = File::create(base.as_ref().with_extension("pagedata"))?;
    writeln!(pagedata_file, "Blank")?;

    Ok(())
}

pub fn i2pdf(img: impl AsRef<Path>, to_gray: bool, alpha: u8) -> Result<()> {
    if alpha > 100 {
        return Err(Error::AlphaRangeError(alpha));
    }

    let output_dir = PathBuf::from("./rem");
    if !output_dir.exists() {
        std::fs::create_dir(&output_dir)?;
    }
    let uu = super::new_uuid();
    let base = create_bare_fs(&uu, &output_dir)?;

    let image = open_image(img.as_ref())?;
    let processed_image = process_image(&image, to_gray, alpha)?;

    let pdf = create_pdf(&img.as_ref().to_string_lossy(), &processed_image);
    let outfile = base.with_extension("pdf");
    pdf.save(&mut BufWriter::new(File::create(outfile)?))?;

    let content_file = File::create(base.with_extension("content"))?;
    let mut content = Content::default();
    let page_uuid = super::new_uuid();
    content.add_page(&page_uuid);
    serde_json::to_writer(content_file, &content)?;

    // TODO: clear out this unwrap.
    create_metadata_file(&img.as_ref().file_name().unwrap().to_string_lossy(), &base)?;
    create_pagedata_file(&base)?;

    let small_image = resize_image(&image, 362, 512);
    write_thumbnail(&small_image, base, &page_uuid)?;

    Ok(())
}
