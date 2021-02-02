use crate::imgtools::process_image;
use crate::remarkable::{create_bare_fs, Content};
use crate::Result;
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;
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

pub fn i2pdf(img: impl AsRef<Path>) -> Result<()> {
    let output_dir = PathBuf::from("./rem");
    if !output_dir.exists() {
        std::fs::create_dir(&output_dir)?;
    }
    let base = create_bare_fs(&output_dir)?;

    let image = open_image(img.as_ref())?;
    let processed_image = process_image(&image, true, 25)?;

    let pdf = create_pdf(&img.as_ref().to_string_lossy(), &processed_image);
    let outfile = base.with_extension("pdf");
    pdf.save(&mut BufWriter::new(File::create(outfile)?))?;

    let content_file = File::create(base.with_extension("content"))?;
    let mut content = Content::default();
    content.add_page("geopage");

    serde_json::to_writer(content_file, &content)?;

    Ok(())
}
