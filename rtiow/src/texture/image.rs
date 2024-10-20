use anyhow::{bail, Result};
use stb_image::image::{self as stbi, LoadResult};
use std::env::var;

#[derive(Debug)]
pub struct RtwImage {
    _fdata: Option<Box<[f32]>>,
    bdata: Box<[u8]>,
    image_size: [usize; 2],
    bytes_per_scanlines: usize,
}

impl<'a> RtwImage {
    const BYTES_PER_PIXEL: usize = 3;
    const DEFAULT_IMAGE_DIR: &'a str = "resources";

    pub fn new(image_filename: &str) -> Result<Self> {
        let image_dir = var("RTW_IMAGES");

        if let Ok(dir) = image_dir {
            if let Ok(ret) = Self::load(format!("{}/{}", dir, image_filename).as_str()) {
                return Ok(ret);
            }
        }
        if let Ok(ret) = Self::load(image_filename) {
            return Ok(ret);
        }
        if let Ok(ret) =
            Self::load(format!("{}/{}", Self::DEFAULT_IMAGE_DIR, image_filename).as_str())
        {
            return Ok(ret);
        }
        if let Ok(ret) =
            Self::load(format!("../{}/{}", Self::DEFAULT_IMAGE_DIR, image_filename).as_str())
        {
            return Ok(ret);
        }
        if let Ok(ret) =
            Self::load(format!("../../{}/{}", Self::DEFAULT_IMAGE_DIR, image_filename).as_str())
        {
            return Ok(ret);
        }
        if let Ok(ret) =
            Self::load(format!("../../../{}/{}", Self::DEFAULT_IMAGE_DIR, image_filename).as_str())
        {
            return Ok(ret);
        }
        if let Ok(ret) = Self::load(
            format!("../../../../{}/{}", Self::DEFAULT_IMAGE_DIR, image_filename).as_str(),
        ) {
            return Ok(ret);
        }
        if let Ok(ret) = Self::load(
            format!(
                "../../../../../{}/{}",
                Self::DEFAULT_IMAGE_DIR,
                image_filename
            )
            .as_str(),
        ) {
            return Ok(ret);
        }

        bail!("error: Could not load image file {}.", image_filename)
    }

    pub fn load(image_filename: &str) -> Result<Self> {
        let data = stbi::load_with_depth(image_filename, Self::BYTES_PER_PIXEL, false);
        let (image_size, bytes_per_scanlines, bdata, fdata) = match data {
            LoadResult::ImageU8(image) => (
                [image.width, image.height],
                image.width * image.depth,
                Box::from(image.data.as_slice()),
                None,
            ),
            LoadResult::ImageF32(image) => (
                [image.width, image.height],
                image.width * image.depth,
                Self::convert_to_bytes(image.data.as_slice()),
                Some(image.data),
            ),
            LoadResult::Error(e) => {
                bail!("{}", e);
            }
        };
        let fdata = if let Some(data) = fdata {
            Some(Box::from(data.as_slice()))
        } else {
            None
        };

        Ok(Self {
            _fdata: fdata,
            bdata,
            image_size,
            bytes_per_scanlines,
        })
    }

    pub fn width(&self) -> usize {
        self.image_size[0]
    }

    pub fn height(&self) -> usize {
        self.image_size[1]
    }

    pub fn pixel_data(&self, x: i32, y: i32) -> &[u8] {
        let x = Self::clamp(x, 0, self.image_size[0] as i32) as usize;
        let y = Self::clamp(y, 0, self.image_size[1] as i32) as usize;

        &self.bdata[y * self.bytes_per_scanlines + x * Self::BYTES_PER_PIXEL..self.bdata.len()]
    }

    fn clamp(x: i32, low: i32, high: i32) -> i32 {
        if x < low {
            low
        } else if x < high {
            x
        } else {
            high - 1
        }
    }

    fn f32_to_byte(value: f32) -> u8 {
        if value <= 0_f32 {
            0
        } else if 1_f32 <= value {
            u8::MAX
        } else {
            (256_f32 * value) as u8
        }
    }

    fn convert_to_bytes(fdata: &[f32]) -> Box<[u8]> {
        let bdata = fdata
            .iter()
            .map(|fbyte| Self::f32_to_byte(*fbyte))
            .collect::<Vec<u8>>();

        Box::from(bdata.as_slice())
    }
}
