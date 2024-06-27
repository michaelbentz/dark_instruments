use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;

use image::DynamicImage;
use which::which;

use crate::error::{
    AdbError,
    CaptureScreenError,
    DisplaySizeError,
    ScreenSumError,
};
use crate::key_code::KeyCode;
use crate::process::Process;
use crate::utils;
use crate::utils::rand_rng;

pub struct DisplaySize {
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug)]
pub struct Adb {
    path_buf: PathBuf,
    target_serial: Option<String>,
}

impl Adb {
    pub fn new(
        adb_path: Option<PathBuf>, target_serial: Option<String>,
    ) -> Result<Self, AdbError> {
        let adb_path_buf = match adb_path {
            Some(provided_adb_path) => provided_adb_path,
            None => match which("adb") {
                Ok(found_adb_path) => found_adb_path,
                Err(_) => {
                    return Err(AdbError::AdbNotFound);
                }
            },
        };
        if !adb_path_buf.exists() {
            return Err(AdbError::AdbNotFound);
        }
        let adb = Adb {
            path_buf: adb_path_buf,
            target_serial: target_serial.clone(),
        };
        match target_serial {
            Some(provided_target_serial) => {
                if !adb.have_target(&provided_target_serial) {
                    return Err(AdbError::TargetNotOnline);
                }
            }
            None => {
                if !adb.have_active_target() {
                    return Err(AdbError::NoExclusiveTargetOnline);
                }
            }
        }
        adb.start_server();
        Ok(adb)
    }

    fn adb_no_target(&self, args: &[&str]) -> Option<(String, String)> {
        Process::exec(self.path_buf.to_string_lossy().as_ref(), args)
    }

    fn adb_no_target_as_bytes(&self, args: &[&str]) -> Option<Vec<u8>> {
        Process::exec_and_return_stdout_bytes(
            self.path_buf.to_string_lossy().as_ref(), args,
        )
    }

    fn adb_target(&self, args: &[&str]) -> Option<(String, String)> {
        match &self.target_serial {
            Some(target) => {
                self.adb_no_target(
                    &["-s", target]
                        .iter()
                        .cloned()
                        .chain(args.iter().cloned())
                        .collect::<Vec<_>>()
                )
            }
            None => {
                self.adb_no_target(args)
            }
        }
    }

    fn adb_target_as_bytes(&self, args: &[&str]) -> Option<Vec<u8>> {
        match &self.target_serial {
            Some(target) => {
                self.adb_no_target_as_bytes(
                    &["-s", target]
                        .iter()
                        .cloned()
                        .chain(args.iter().cloned())
                        .collect::<Vec<_>>()
                )
            }
            None => {
                self.adb_no_target_as_bytes(args)
            }
        }
    }

    pub fn restart_server(&self) {
        self.kill_server();
        self.start_server();
        sleep(Duration::from_millis(
            rand_rng(1000, 3000)
        ));
    }

    pub fn kill_server(&self) {
        self.adb_no_target(&["kill-server"]);
    }

    pub fn start_server(&self) {
        self.adb_no_target(&["start-server"]);
    }

    fn targets(&self) -> Option<(String, String)> {
        self.adb_no_target(&["devices"])
    }

    pub(crate) fn have_active_target(&self) -> bool {
        match self.targets() {
            Some((output, _)) => {
                let matched_lines = output.lines().filter(
                    |line| line.trim().ends_with("device") && line.split_whitespace().count() > 1
                );
                matched_lines.take(2).count() == 1
            }
            None => false
        }
    }

    pub fn have_target(&self, target: &str) -> bool {
        self.targets().map_or(
            false, |output| output.0.contains(target),
        )
    }

    pub fn display_size(&self) -> Result<DisplaySize, DisplaySizeError> {
        if let Some((stdin_output, _)) = self.adb_target(&["shell", "wm", "size"]) {
            let size_as_string: String = stdin_output
                .chars()
                .filter(|&c| c.is_ascii_digit() || c == 'x')
                .collect();

            let mut split_iter = size_as_string.split('x');
            if let (Some(width_str), Some(height_str)) = (
                split_iter.next(), split_iter.next()
            ) {
                if let (Ok(width), Ok(height)) = (
                    width_str.parse::<u32>(), height_str.parse::<u32>()
                ) {
                    return Ok(DisplaySize { width, height });
                }
            }
        }
        Err(DisplaySizeError)
    }

    pub fn capture_screen_as_bytes(&self) -> Result<Vec<u8>, CaptureScreenError> {
        if let Some(output) = self.adb_target_as_bytes(
            &["exec-out", "screencap", "-p"]
        ) {
            return Ok(output);
        }
        Err(CaptureScreenError)
    }

    pub fn capture_screen_as_dynamic_image(&self) -> Result<DynamicImage, CaptureScreenError> {
        let image_bytes = self.capture_screen_as_bytes()?;
        image::load_from_memory(&image_bytes)
            .map_err(|_| CaptureScreenError)
    }

    pub fn capture_screen_as_file(&self, file_name: &str) -> Result<PathBuf, CaptureScreenError> {
        let dynamic_image = self.capture_screen_as_dynamic_image()?;
        utils::save_dynamic_image_as_png(&dynamic_image, file_name)
            .map_err(|_| CaptureScreenError)
    }

    pub fn screen_sum(&self) -> Result<String, ScreenSumError> {
        if let Ok(bytes) = self.capture_screen_as_bytes() {
            return Ok(utils::md5_of_bytes(&bytes));
        }
        Err(ScreenSumError)
    }

    pub fn start_activity(&self, activity_component_name: &str) -> Option<(String, String)> {
        self.adb_target(&["shell", "am", "start", "-n", activity_component_name])
    }

    pub fn input_shown(&self) -> bool {
        match self.adb_target(
            &["shell", "dumpsys", "input_method", "|", "grep", "mInputShown"],
        ) {
            Some((output, _)) => {
                output.contains("mInputShown=true")
            }
            None => false
        }
    }

    pub fn input_text(&self, text: &str) -> Option<(String, String)> {
        self.adb_target(&["shell", "input", "text", text])
    }

    pub fn input_key_event(&self, key_code: KeyCode) -> Option<(String, String)> {
        self.adb_target(&["shell", "input", "keyevent", &*(key_code as u32).to_string()])
    }

    pub fn input_tap(&self, x: u32, y: u32) -> Option<(String, String)> {
        self.adb_target(&["shell", "input", "tap", &x.to_string(), &y.to_string()])
    }

    pub fn input_swipe(
        &self,
        x_start: u32, x_end: u32,
        y_start: u32, y_end: u32,
        duration: u32,
    ) -> Option<(String, String)> {
        self.adb_target(&[
            "shell", "input", "touchscreen", "swipe",
            &x_start.to_string(),
            &y_start.to_string(),
            &x_end.to_string(),
            &y_end.to_string(),
            &duration.to_string()
        ])
    }
}