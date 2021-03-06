use crate::Result;
use printpdf::image::imageops;
use printpdf::image::{DynamicImage, ImageBuffer, Luma, Pixel, Primitive, Rgb};
use std::borrow::Cow;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ColorTransform {
    ToBlackAndWhite,
    ToGrayscale,
    None,
}

trait MulAlpha
where
    Self: Pixel + 'static,
    Self::Subpixel: Primitive + 'static,
{
    /// Pre-multiplies a pixel by the requested `alpha` value. Values for alpha should be
    /// in the range (0.0..1.0), with 0.0 being fully transparent, and 1.0 being fully opaque.
    fn mul_alpha(&self, alpha: f32) -> Self;

    /// Pre-multiplies an ImageBuffer by the requested 'alpha` value. Values for alpha should be
    /// in the range (0.0..1.0), with 0.0 being fully transparent, and 1.0 being fully opaque.
    fn mul_alpha_buffer(
        img: &ImageBuffer<Self, Vec<Self::Subpixel>>,
        alpha: f32,
    ) -> ImageBuffer<Self, Vec<Self::Subpixel>> {
        let (width, height) = img.dimensions();
        let mut out = ImageBuffer::new(width, height);

        for y in 0..height {
            for x in 0..width {
                let pixel = img.get_pixel(x, y).mul_alpha(alpha);
                out.put_pixel(x, y, pixel);
            }
        }

        out
    }
}

impl<S: Primitive + std::fmt::Debug + 'static> MulAlpha for Luma<S> {
    fn mul_alpha(&self, alpha: f32) -> Self {
        use num_traits::cast;
        self.map_with_alpha(
            |p| {
                // unwrap: S fits into an f32.
                let max_pixel: f32 = cast(S::max_value()).unwrap();
                let bgrnd: f32 = (1.0 - alpha) * max_pixel;
                // unwrap: S fits into an f32.
                let p_as_f32: f32 = cast(p).unwrap();
                let fgrnd: f32 = alpha * p_as_f32;
                // unwrap: computed value *should* fit into S.
                cast(bgrnd + fgrnd).unwrap()
            },
            |_| S::max_value(),
        )
    }
}

impl<S: Primitive + std::fmt::Debug + 'static> MulAlpha for Rgb<S> {
    fn mul_alpha(&self, alpha: f32) -> Self {
        use num_traits::cast;
        self.map_with_alpha(
            |p| {
                // unwrap: S fits into an f32.
                let max_pixel: f32 = cast(S::max_value()).unwrap();
                let bgrnd: f32 = (1.0 - alpha) * max_pixel;
                // unwrap: S fits into an f32.
                let p_as_f32: f32 = cast(p).unwrap();
                let fgrnd: f32 = alpha * p_as_f32;
                // unwrap: computed value *should* fit into S.
                cast(bgrnd + fgrnd).unwrap()
            },
            |_| S::max_value(),
        )
    }
}

/// Dispatches on DynamicImage and applies an alpha transform, returning a new image.
fn mul_alpha_to_image(img: &DynamicImage, alpha: f32) -> DynamicImage {
    match img {
        DynamicImage::ImageLuma8(buffer) => {
            DynamicImage::ImageLuma8(MulAlpha::mul_alpha_buffer(buffer, alpha))
        }
        DynamicImage::ImageRgb8(buffer) => {
            DynamicImage::ImageRgb8(MulAlpha::mul_alpha_buffer(buffer, alpha))
        }
        DynamicImage::ImageRgb16(buffer) => {
            DynamicImage::ImageRgb16(MulAlpha::mul_alpha_buffer(buffer, alpha))
        }
        _ => unimplemented!("add_alpha_to_image"),
    }
}

/// Returns a (possibly new) image with the requested transform applied.
///
/// The resulting image is "pre-multiplied" (since the Remarkable seems to do
/// weird things otherwise), and the alphas in the final image, if present,
/// will be set tofully opaque.
///
/// If `to_gray` is true, the image will be converted to grayscale.
///
/// If `alpha` is true, the image will be multiplied by its value. Legal values are
/// `0-100`, where 100 = opaque, 0 = fully transparent.
pub fn process_image(
    img: &DynamicImage,
    color_transform: ColorTransform,
    alpha: u8,
) -> Result<Cow<DynamicImage>> {
    let mut output = Cow::Borrowed(img);

    match color_transform {
        ColorTransform::ToGrayscale => {
            let temp = DynamicImage::ImageLuma8(imageops::grayscale(output.as_ref()));
            output = Cow::Owned(temp);
        }
        ColorTransform::ToBlackAndWhite => {
            // TODO: have a way to dial in the exact threshold for B/W.
            let mut gray = imageops::grayscale(output.as_ref());
            imageops::colorops::dither(&mut gray, &imageops::colorops::BiLevel);
            output = Cow::Owned(DynamicImage::ImageLuma8(gray));
        }
        ColorTransform::None => {}
    }

    if alpha < 100 {
        let alpha = f32::from(alpha) / 100.0;
        let temp = mul_alpha_to_image(&output, alpha);
        output = Cow::Owned(temp);
    }

    Ok(output)
}
