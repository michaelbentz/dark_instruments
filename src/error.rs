use std::{fmt, io};
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum AdbError {
    AdbNotFound,
    TargetNotOnline,
    NoExclusiveTargetOnline,
}

impl Display for AdbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            AdbError::AdbNotFound => write!(f, "ADB binary not found"),
            AdbError::TargetNotOnline => write!(f, "Provided target is not online"),
            AdbError::NoExclusiveTargetOnline => write!(f, "No exclusive target online"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DisplaySizeError;

impl Display for DisplaySizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to get display size")
    }
}

impl Error for DisplaySizeError {}

#[derive(Debug, Clone)]
pub struct CaptureScreenError;

impl Display for CaptureScreenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to capture screen")
    }
}

impl Error for CaptureScreenError {}

#[derive(Debug, Clone)]
pub struct ScreenSumError;

impl Display for ScreenSumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to calculate screen sum")
    }
}

impl Error for ScreenSumError {}

#[derive(Debug)]
pub enum ImageSaveError {
    Io(io::Error),
    ImageSaveFailed,
}

impl Display for ImageSaveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImageSaveError::Io(err) => write!(f, "IO error: {}", err),
            ImageSaveError::ImageSaveFailed => write!(f, "Failed to save image"),
        }
    }
}

impl Error for ImageSaveError {}