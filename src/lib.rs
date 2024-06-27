pub use crate::dark_instruments::Args;
pub use crate::dark_instruments::DarkInstruments;

mod dark_instruments;
mod process;
mod adb;
mod key_code;
mod tesseract;
mod utils;
mod error;

pub mod bridge {
    pub use crate::adb::Adb;
    pub use crate::adb::DisplaySize;
    pub use crate::key_code::KeyCode;

    pub mod error {
        pub use crate::error::AdbError;
        pub use crate::error::CaptureScreenError;
        pub use crate::error::DisplaySizeError;
        pub use crate::error::ScreenSumError;
    }
}

pub mod toolkit {
    pub mod image {
        pub use crate::tesseract::TesseractImage;
        pub use crate::utils::save_dynamic_image_as_png;
    }

    pub mod digest {
        pub use crate::utils::md5_of_bytes;
        pub use crate::utils::md5_of_file;
    }

    pub mod rand {
        pub use crate::utils::rand_rng;
    }

    pub mod error {
        pub use crate::error::ImageSaveError;
    }
}

#[cfg(test)]
mod tests;