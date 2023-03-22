use core::time;
use std::{fs::OpenOptions, io::Write};

use rustpython_vm as vm;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use vm::{
    compiler::{CodeObject, CompileError, CompileOpts},
    VirtualMachine,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct CodeReader {
    string_frames: Vec<Frame>,
    last_update_time: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Frame {
    string: String,
    compiled: bool,
}

impl CodeReader {
    pub fn new() -> Self {
        Self {
            string_frames: Vec::new(),
            last_update_time: Utc::now().timestamp() as u64,
        }
    }

    pub fn add_string_state_frame(&mut self, string_frame: String) {
        // If the last update was more than 1 second ago, write to file
        if Utc::now().timestamp() as u64 - self.last_update_time > 1 {}
        self.write_to_file();

        vm::Interpreter::without_stdlib(Default::default()).enter(|vm| {
            let scope = vm.new_scope_with_builtins();

            let py_file = &format!("{}\n\n\n{}", &string_frame, include_str!("test.py"));
            CodeReader::log(format!("{:?}", py_file));


            let code_obj = vm
                .compile(
                    py_file,
                    vm::compiler::Mode::Exec,
                    "<embedded>".to_owned(),
                )
                .map_err(|err| vm.new_syntax_error(&err));

            // If it's an error, log it then return
            if let Err(err) = code_obj {
                // CodeReader::log(format!("{:?}", err.traceback()));
                return;
            }

            let code_obj = code_obj.unwrap();

            let run_result = vm.run_code_obj(code_obj, scope);
            CodeReader::log(format!("{:?}", run_result));
            match run_result {
                Ok(result) => {
                    CodeReader::log(format!("{:?}", result));
                    panic!("Code compiled")
                }
                Err(err) => {
                    CodeReader::log(format!("{:?}", err.traceback()));
                }
            }

            // vm::PyResult::Ok(())
        });

        // Try compiling the code
        // let compiled = match compile_result {
        //     Ok(_) => true,
        //     Err(_) => false,
        // };

        // // If we compiled the code, panic
        // if compiled {
        //     panic!("Code compiled");
        // }

        // let frame = Frame {
        //     string: string_frame,
        //     compiled: compiled,
        // };

        // // Push the frame to the vector

        // self.string_frames.push(frame);

        // Set the current time as the last update time
        self.last_update_time = Utc::now().timestamp() as u64;
    }

    pub fn write_to_file(&self) {
        let output_string = serde_json::to_string_pretty(&self).unwrap();

        // Write to a file in the current directory
        let mut file = std::fs::File::create("code_reader_state.json").unwrap();

        file.write_all(output_string.as_bytes()).unwrap();
    }

    pub fn log(string: String) {
        // Append this string to the log file
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("ox.log")
            .unwrap();

        file.write_all(string.as_bytes()).unwrap();
        // Write a newline
        file.write_all(b"\n").unwrap();
    }
}
