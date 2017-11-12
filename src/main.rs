extern crate clang;

mod clang_rs_ext;

use clang::*;
use std::env;
use clang_rs_ext::*;


fn main() {
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
        .skip_function_bodies(true)
        .incomplete(true)
        .keep_going(true)
        .single_file_parse(false)
        .parse()
        .unwrap();

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

    entities.into_iter().for_each(|e| e.print_info());
}
