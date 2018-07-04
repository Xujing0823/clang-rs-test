extern crate clang;

use clang::*;
// use std::env;
use std::fmt;
use std::fmt::Debug;


pub trait EntityExt {
    fn get_file_path(&self) -> Option<String>;
    fn get_file_line(&self) -> Option<u32>;
    fn print_info(&self);
}

pub trait DisplayBridge {
    fn fmt_bridge(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

impl fmt::Display for DisplayBridge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_bridge(f)
    }
}

impl DisplayBridge for clang::Accessibility {
    fn fmt_bridge(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
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
        let print_info_general = || {
            println!("{} : {:?}, type: {}, Location: {}:{}"
                     , self.get_name().unwrap_or(String::from("!!!Unknow!!!"))
                     , self.get_kind()
                     , self.get_type()
                         .map(|e| e.get_display_name())
                         .unwrap_or(String::from("!!!Unknow!!!"))
                     , self.get_file_path()
                         .unwrap_or(String::from("!!!Unprintable!!!"))
                     , self.get_file_line()
                         .map(|u| u as i64)
                         .unwrap_or(-1)
            )
        };
        match self.get_kind() {
            EntityKind::ClassDecl | EntityKind::ClassTemplate | EntityKind::ClassTemplatePartialSpecialization => {
                print_info_general();
                self.get_children().iter().for_each(|e| e.print_info());
            },
            EntityKind::Method => {
                println!("{}::{} : {:?}, type: {}, Accessibility: {}, Location: {}:{}"
                         , self.get_semantic_parent()
                             .and_then(|e| e.get_name())
                             .unwrap_or(String::from(""))
                         , self.get_name().unwrap_or(String::from("!!!Unknow!!!"))
                         , self.get_kind()
                         , self.get_type()
                             .map(|e| e.get_display_name())
                             .unwrap_or(String::from("!!!Unknow!!!"))
                         , self.get_accessibility()
                             .map(|e| (&e as &DisplayBridge).to_string())
                             .unwrap_or(String::from("!!!Unknow!!!"))
                         , self.get_file_path()
                             .unwrap_or(String::from("!!!Unprintable!!!"))
                         , self.get_file_line()
                             .map(|u| u as i64)
                             .unwrap_or(-1)
                );
            },
            _ => {
                print_info_general();
            }
        }
    }
}
