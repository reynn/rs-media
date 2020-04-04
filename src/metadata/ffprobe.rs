use regex::Regex;
use std::borrow::Borrow;
use std::{
    io::{self, Error as stdError, ErrorKind},
    process::Command,
    time::Duration,
};

pub enum Error {}

#[derive(Debug, Clone)]
pub struct MetaData {
    pub file_name: String,
    pub format: Option<String>,
    pub created: Option<String>,
    pub duration: Option<std::time::Duration>,
    pub start: Option<String>,
    pub bit_rate: Option<String>,
    pub streams: Option<Vec<String>>,
    raw_output_lines: Vec<String>,
}

lazy_static! {
    static ref RE_DURATION_LINE: Regex =
        Regex::new(r"^  Duration: (?P<dur>.+?), start: (?P<start>.+?), bitrate: (?P<brate>.+?)$")
            .unwrap();
}

impl Default for MetaData {
    fn default() -> MetaData {
        MetaData {
            streams: None,
            file_name: String::new(),
            bit_rate: None,
            format: None,
            created: None,
            duration: Some(Duration::from_secs(0)),
            start: None,
            raw_output_lines: Vec::new(),
        }
    }
}

impl MetaData {
    pub fn new(file_name: &str, probe_bin: &str) -> io::Result<Self> {
        let res_a = Command::new(probe_bin)
            .arg("-show_streams")
            .arg(file_name)
            .output()
            .unwrap();

        if !res_a.status.success() {
            Err(stdError::new(
                ErrorKind::InvalidData,
                format!(
                    "Failed to gather data on {}, status code: {}",
                    file_name.to_string(),
                    res_a.status.code().unwrap()
                ),
            ))
        } else {
            let raw_lines: String =
                String::from_utf8(res_a.stderr).expect("Unable to parse the ffprobe output");
            MetaData::from_utf8(raw_lines.borrow())
        }
    }

    pub fn from_utf8(_utf8_data: &String) -> io::Result<Self> {
        // utf8_data.split("\n")
        //     .map(|s| s.into())
        //     .collect();
        Ok(MetaData::default())
    }
}
