use std::cmp::Ordering;
use std::env;

pub struct Args {
    pub should_compress: bool,
    pub input_file: String,
    pub output_file: Option<String>,
    pub is_dir: bool,
    pub iter: Option<u8>,
}

pub fn get_args() -> Args {
    let argv = env::args();
    if argv.len() < 3 {
        panic!("Not enough arguments, {} provided!", argv.len());
    }

    let mut arg_out = Args {
        should_compress: false,
        input_file: String::from(""),
        output_file: None,
        is_dir: false,
        iter: None,
    };

    let mut argv_iter = argv.into_iter();
    while let Some(a) = argv_iter.next() {
        if a.cmp(&String::from("compress")) == Ordering::Equal
            || a.cmp(&String::from("c")) == Ordering::Equal
        {
            arg_out.should_compress = true;
        } else if a.cmp(&String::from("decompress")) == Ordering::Equal
            || a.cmp(&String::from("c")) == Ordering::Equal
        {
            arg_out.should_compress = false;
        } else {
            if a.cmp(&String::from("-o")) == Ordering::Equal {
                arg_out.output_file = argv_iter.next();
            } else if a.cmp(&String::from("-i")) == Ordering::Equal {
                match argv_iter.next().unwrap().parse::<u8>() {
                    Ok(i) => arg_out.iter = Some(i),
                    Err(_) => arg_out.iter = None,
                }
            } else {
                arg_out.input_file = a;
            }
        }
    }

    arg_out
}
