extern crate clang;

mod clang_rs_ext;
mod duration_ext;

use clang::*;
use std::env;
use clang_rs_ext::*;
use duration_ext::*;
use std::time;

fn main() {
    let start_time = time::Instant::now();
    let args: Vec<String> = env::args().collect();

    let path;

    if args.len() == 2 {
        path = args[1].clone();
    } else {
        path = String::from("structs.c");
    }

    println!("file: {}", path);

    // Acquire an instance of `Clang`
    let clang = Clang::new().unwrap();

    // Create a new `Index`
    let index = Index::new(&clang, false, false);

    // Parse a source file into a translation unit
    let tu: TranslationUnit = index.parser(&path)
        .arguments(&["-xc++",
            "-std=c++11",
            "-I/Users/xujing/Workspace/rtags/src",
            "-I/Users/xujing/Workspace/rtags/src/rct",
            "-I/Users/xujing/Workspace/rtags/build/src/include",
            "-I/Users/xujing/Workspace/rtags/src/selene/include",
            "-I/Users/xujing/Workspace/rtags/build/src/lua-prefix/src/lua-build",
            "-I/Users/xujing/Workspace/rtags/src/lua/src",
            "-I/usr/local/Cellar/llvm/5.0.0/include"])
        .cache_completion_results(false)
        .detailed_preprocessing_record(true)
        .briefs_in_completion_results(false)
        .skip_function_bodies(true)
        .incomplete(true)
        .keep_going(true)
        .single_file_parse(false)
        .parse()
        .unwrap();

    let parse_duration = start_time.elapsed();
    let start_filter = time::Instant::now();

    let entities: Vec<Entity> = tu.get_entity().get_children().into_iter().filter(|e: &Entity| {
        let source_file_path = e.get_location()
            .map(|source_location| source_location.get_file_location())
            .and_then(|location| location.file)
            .map(|file| file.get_path());
        if source_file_path != None
            && source_file_path.unwrap() == std::path::PathBuf::from(&path) {
            true
        } else {
            false
        }
    }).collect::<Vec<_>>();

    let filter_duration = start_filter.elapsed();
    let start_print = time::Instant::now();

    entities.into_iter().for_each(|e| e.print_info());

    println!("");
    println!("Parse consumed {} ms, filter consumed {} ms, print consumed {} ms"
             , parse_duration.to_millis()
             , filter_duration.to_millis()
             , start_print.elapsed().to_millis());
    println!("");

}
