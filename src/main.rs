extern crate clang;

use clang::*;
use std::env;

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
    let tu: TranslationUnit = index.parser(&path).parse().unwrap();

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

    entities.into_iter().for_each(|e| {
       println!("e = {:?}", e);
    });
}
