use std::process::Command;
use std::io::{self, Write};
use std::fs::File;
use std::io::Read;
use serde::Deserialize;
extern crate windows_sys;
use core::ffi::c_void;
use windows_sys::Win32::System::Threading::OpenProcess;
extern crate winapi;
use windows_sys::Win32::System::Diagnostics::Debug::WriteProcessMemory;
use windows_sys::Win32::System::Threading::PROCESS_ALL_ACCESS;


#[derive(Debug, Deserialize)]
struct Data { pid: u32,}

fn main() {
    execute_pid();
    read_json();
    //write_memory();
}

fn execute_pid(){
    // execute c++ script
    let output = Command::new("C:/Users/nico/Documents/memory_cheat/src/getPID.exe")
        .output()
        .expect("error by executing pid script");

    // print script output
    let _pid = io::stdout().write_all(&output.stdout).unwrap();
    
    // verify if there is any error
    if !output.status.success() {
        eprintln!("error by executing pid script");
    }
}

fn read_json(){
    // try to open json
    let file_result = File::open("info.json");
    
    match file_result {
        Ok(mut file) => {
            // read
            let mut json_data = String::new();
            file.read_to_string(&mut json_data).expect("error reading file");

            // deserialize
            let result: Result<Data, _> = serde_json::from_str(&json_data);

            match result {
                Ok(data) => {
                    println!("got pid successfully");
                    user_input(data.pid);
                    //
                }
                Err(e) => {
                    eprintln!("error reading file: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("error reading file: {}", e);
        }
    }
}


fn write_memory(w_pid: u32, user_input: u32){
    let handle = unsafe { OpenProcess(PROCESS_ALL_ACCESS, 0, w_pid)};
    let addr_pointer = 0x69811EBA4 as *const c_void;
    let val_to_write: u32 = user_input;
    let val_pointer = &val_to_write as *const u32;

    loop{
        unsafe {
            let mut bytes_written: usize = 0;
            let _result = WriteProcessMemory(
                handle,
                addr_pointer,
                val_pointer as *const c_void,
                std::mem::size_of::<u32>(),
                &mut bytes_written,
            );
        }
    }
}

fn user_input(w_pid: u32){
    loop {
        println!("");
        println!("[+] value: ");
        io::stdout().flush().unwrap(); 

        let mut input_text = String::new();

        match io::stdin().read_line(&mut input_text) {
            Ok(_) => {
                match input_text.trim().parse::<u32>() {
                    Ok(number) => {
                        write_memory(w_pid, number);
                        break;
                    }
                    Err(_) => {
                        println!("input only a number please");
                    }
                }
            }
            Err(error) => {
                println!("error reading entry: {}", error);
                break;
            }
        }
    }
}