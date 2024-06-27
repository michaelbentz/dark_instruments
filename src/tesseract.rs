use std::collections::HashMap;
use std::path::PathBuf;

use image::{
    DynamicImage,
    ImageError,
};
use rusty_tesseract::{
    Args, DataOutput, Image,
};

const TESSERACT_CONFIG_VAR_NAME_CHAR_WHITELIST: &str = "tessedit_char_whitelist";
const TESSERACT_LANG: &str = "eng";
const TESSERACT_DPI: i32 = 120;

pub struct TesseractImage {
    dynamic_image: DynamicImage,
}

impl TesseractImage {
    pub fn from_dynamic_image(dynamic_image: DynamicImage) -> Self {
        TesseractImage { dynamic_image }
    }

    pub fn from(path_buf: PathBuf) -> Result<Self, ImageError> {
        let dynamic_image = image::open(path_buf)?;
        Ok(TesseractImage { dynamic_image })
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, ImageError> {
        let dynamic_image = image::load_from_memory(&bytes)?;
        Ok(TesseractImage { dynamic_image })
    }

    fn data_output(&self, text: &str) -> Result<DataOutput, ()> {
        if let Ok(image) = Image::from_dynamic_image(&self.dynamic_image) {
            let args = Args {
                lang: String::from(TESSERACT_LANG),
                dpi: Some(TESSERACT_DPI),
                config_variables: HashMap::from([
                    (String::from(TESSERACT_CONFIG_VAR_NAME_CHAR_WHITELIST), String::from(text))
                ]),
                psm: None,
                oem: None,
            };
            return Ok(rusty_tesseract::image_to_data(&image, &args).unwrap());
        }
        Err(())
    }

    pub fn contains_text(&self, text: &str) -> bool {
        if let Ok(data_output) = self.data_output(text) {
            data_output.data.iter().any(|data| data.text == text)
        } else {
            false
        }
    }

    pub fn xy_positions_of(&self, text: &str) -> Option<(Vec<i32>, Vec<i32>)> {
        if let Ok(data_output) = self.data_output(text) {
            let xy_positions: (Vec<i32>, Vec<i32>) = data_output
                .data
                .iter()
                .filter(|data| text == data.text)
                .map(|data| (data.left + (data.width / 2), data.top + (data.height / 2)))
                .unzip();

            Some(xy_positions)
        } else {
            None
        }
    }
}