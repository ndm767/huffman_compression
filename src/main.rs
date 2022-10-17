mod binary_tree;
mod compress;
mod decompress;

use compress::compress;
use decompress::decompress;
use std::env;

fn main() {
    let mut argv = env::args();
    if argv.len() < 3 {
        panic!("Not enough arguments, {} provided!", argv.len());
    }

    let should_compress = argv.nth(1).unwrap().starts_with("compress");
    if should_compress {
        let input_file = argv.nth(0).unwrap();
        let output_file = if argv.len() > 0 {
            argv.nth(0).unwrap()
        } else {
            String::from("out.hfm")
        };

        compress(input_file, output_file);
    } else {
        let input_file = argv.nth(0).unwrap();
        let output_file: Option<String> = if argv.len() > 0 {
            Some(argv.nth(0).unwrap())
        } else {
            None
        };
        decompress(input_file, output_file);
    }
}
