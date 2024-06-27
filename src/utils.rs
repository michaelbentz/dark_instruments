use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

use image::{
    DynamicImage,
    ImageFormat,
};
use md5::{Digest, Md5};
use rand::Rng;

use crate::error::ImageSaveError;

pub fn md5_of_file(path_buf: &PathBuf) -> io::Result<String> {
    let mut file = File::open(path_buf)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(md5_of_bytes(&buffer))
}

pub fn md5_of_bytes(bytes: &[u8]) -> String {
    let mut md5_core = Md5::new();
    md5_core.update(bytes);
    format!("{:x}", md5_core.finalize())
}

pub fn rand_rng(min: u64, max: u64) -> u64 {
    assert!(min <= max);
    rand::thread_rng().gen_range(min..=max)
}

pub fn save_dynamic_image_as_png(
    dynamic_image: &DynamicImage, file_name: &str,
) -> Result<PathBuf, ImageSaveError> {
    let path_buf = PathBuf::from(file_name);
    dynamic_image
        .save_with_format(&path_buf, ImageFormat::Png)
        .map_err(|_| ImageSaveError::ImageSaveFailed)
        .map(|_| path_buf.clone())
}