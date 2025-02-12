#![expect(clippy::print_stdout)]

use std::fs;
use std::path::Path;
use oxc_allocator::Allocator;
use oxc_parser::{ ParseOptions, Parser };
use oxc_span::SourceType;

fn main() -> Result<(), String> {
    let name = "node_modules/localforage/dist/localforage.js";

    let path = Path::new(name);

    let source_text = fs::read_to_string(path).map_err(|_| format!("Missing '{name}'"))?;

    let source_type = SourceType::from_path(path).unwrap();

    let allocator = Allocator::default();

    let ret = Parser::new(&allocator, &source_text, source_type)
        .with_options(ParseOptions::default())
        .parse();

    if ret.errors.is_empty() {
        println!("Parsed Successfully.");
        if (!ret.module_record.has_module_syntax) {
            // UMD?  AMD? CommonJS?
            
        }
    } else {
        for error in ret.errors {
            let error = error.with_source_code(source_text.clone());
            println!("{error:?}");
            println!("Parsed with Errors.");
        }
    }

    Ok(())
}
