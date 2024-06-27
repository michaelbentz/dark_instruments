use crate::{Args, DarkInstruments};
use crate::adb::Adb;

#[test]
fn test_dark_instruments_adb() {
    let args = Args {
        adb_path: None
    };
    let dark_instruments = DarkInstruments::new(args);
    let result = dark_instruments.adb(None);
    match result {
        Ok(adb) => {
            println!("Have ADB {:?}", adb)
        }
        Err(err) => {
            println!("AdbError {}", err)
        }
    }
}

#[test]
fn test_adb() {
    match Adb::new(None, None) {
        Ok(adb) => {
            assert!(adb.have_active_target());
        }
        Err(err) => {
            println!("AdbError {err}")
        }
    }
}