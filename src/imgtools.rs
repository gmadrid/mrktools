use crate::Result;
use printpdf::image::imageops;
use printpdf::image::{DynamicImage, ImageBuffer, Luma, Pixel, Primitive, Rgb};
use std::borrow::Cow;

trait MulAlpha
where
    Self: Pixel + 'static,
    Self::Subpixel: Primitive + 'static,
{
    fn mul_alpha(&self, alpha: f32) -> Self;

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
        self.map_with_alpha(
            |p| {
                let max_pixel: f32 = num_traits::NumCast::from(S::max_value()).unwrap();
                let bgrnd: f32 = (1.0 - alpha) * max_pixel;
                let p_as_f32: f32 = num_traits::NumCast::from(p).unwrap();
                let fgrnd: f32 = alpha * p_as_f32;
                num_traits::NumCast::from(bgrnd + fgrnd).unwrap()
            },
            |_| S::max_value(),
        )
    }
}

impl<S: Primitive + std::fmt::Debug + 'static> MulAlpha for Rgb<S> {
    fn mul_alpha(&self, alpha: f32) -> Self {
        self.map_with_alpha(
            |p| {
                let max_pixel: f32 = num_traits::NumCast::from(S::max_value()).unwrap();
                let bgrnd: f32 = (1.0 - alpha) * max_pixel;
                let p_as_f32: f32 = num_traits::NumCast::from(p).unwrap();
                let fgrnd: f32 = alpha * p_as_f32;
                num_traits::NumCast::from(bgrnd + fgrnd).unwrap()
            },
            |_| S::max_value(),
        )
    }
}

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

pub fn process_image(img: &DynamicImage, to_gray: bool, alpha: u8) -> Result<Cow<DynamicImage>> {
    let mut output = Cow::Borrowed(img);

    if to_gray {
        let temp = DynamicImage::ImageLuma8(imageops::grayscale(output.as_ref()));
        output = Cow::Owned(temp);
    }

    if alpha < 100 {
        let alpha = f32::from(alpha) / 100.0;
        let temp = mul_alpha_to_image(&output, alpha);
        output = Cow::Owned(temp);
    }

    Ok(output)
}
