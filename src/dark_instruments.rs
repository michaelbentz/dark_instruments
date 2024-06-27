use std::path::PathBuf;

use crate::bridge::Adb;
use crate::error::AdbError;

#[derive(Clone, Debug, PartialEq)]
pub struct Args {
    pub adb_path: Option<PathBuf>,
}

#[derive(Clone)]
pub struct DarkInstruments {
    pub args: Args,
}

impl DarkInstruments {
    pub fn new(args: Args) -> Self {
        DarkInstruments { args }
    }

    pub fn adb(&self, target: Option<String>) -> Result<Adb, AdbError> {
        Adb::new(
            self.args.adb_path.clone(),
            target,
        )
    }
}