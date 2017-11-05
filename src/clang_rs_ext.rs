extern crate clang;

use clang::*;
use std::env;


pub trait EntityExt {
    fn get_file_path(&self) -> Option<String>;
    fn get_file_line(&self) -> Option<u32>;
    fn print_info(&self);
}

impl<'tu> EntityExt for Entity<'tu> {
    fn get_file_path(&self) -> Option<String> {
        self.get_location()
            .map(|source_location| source_location.get_file_location())
            .and_then(|location| location.file)
            .and_then(|file| file.get_path().into_os_string().into_string().ok())
    }
    fn get_file_line(&self) -> Option<u32> {
        self.get_location()
            .map(|source_location| source_location.get_file_location())
            .map(|location| location.line)
    }
    fn print_info(&self) {
        println!("[{}] kind: {:?}, id: {}, type: {}, {}"
                 , self.get_file_line()
                     .map(|u| u as i64)
                     .unwrap_or(-1)
                 , self.get_kind()
                 , self.get_name().unwrap_or(String::from("!!!Unknow!!!"))
                 , self.get_type()
                     .map(|e| e.get_display_name())
                     .unwrap_or(String::from("!!!Unknow!!!"))
                 , self.get_file_path()
                     .unwrap_or(String::from("!!!Unprintable!!!"))
        );
    }
}
