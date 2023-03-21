use core::time;
use std::io::Write;

use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CodeReader {
    string_frames: Vec<String>,
    last_update_time: u64,
}

impl CodeReader {
    pub fn new() -> Self {
        Self {
            string_frames: Vec::new(),
            last_update_time: Utc::now().timestamp() as u64,
        }
    }

    pub fn add_string_state_frame(&mut self, string_frame: String) {
        self.string_frames.push(string_frame);

        // If the last update was more than 1 second ago, write to file
        if Utc::now().timestamp() as u64 - self.last_update_time > 1 {
            self.write_to_file();
        }

        // Set the current time as the last update time
        self.last_update_time = Utc::now().timestamp() as u64;
    }

    pub fn write_to_file(&self) {
        let output_string = serde_json::to_string(&self).unwrap();

        // Write to a file in the current directory
        let mut file = std::fs::File::create("code_reader_state.json").unwrap();

        file.write_all(output_string.as_bytes()).unwrap();
    }
}
