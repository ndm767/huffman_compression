use crate::arg_handler::Args;

use std::{cmp::Ordering, fs};

static SEPARATOR_UPPER: u8 = 0b01010101;
static SEPARATOR_LOWER: u8 = 0b10101010;

pub fn decompress(args: Args) {
    let input_file = args.input_file;

    let in_bytes = fs::read(input_file.clone()).unwrap();
    let in_len = in_bytes.len();
    println!("input len: {}", in_len);

    let mut in_bytes_iter = in_bytes.into_iter();
    let mut curr_byte = in_bytes_iter.next().unwrap();

    // get original file name
    let mut orig_file_name = String::from("");
    loop {
        let next_byte = in_bytes_iter.next().unwrap();
        if curr_byte == SEPARATOR_UPPER && next_byte == SEPARATOR_LOWER {
            break;
        } else {
            orig_file_name.push(curr_byte as char);
            curr_byte = next_byte;
        }
    }
    curr_byte = in_bytes_iter.next().unwrap();

    println!("Original file name: {}", orig_file_name);
    let output_file = match args.output_file {
        Some(o) => o,
        None => orig_file_name,
    };

    // get encoding
    let mut lookup: Vec<Vec<(String, u8)>> = Vec::new();
    loop {
        let symbol = curr_byte;
        let code_len = in_bytes_iter.next().unwrap();
        let code_entries: [u8; 4] = [
            in_bytes_iter.next().unwrap(),
            in_bytes_iter.next().unwrap(),
            in_bytes_iter.next().unwrap(),
            in_bytes_iter.next().unwrap(),
        ];
        let mut code_num = u32::from_be_bytes(code_entries);

        let mut code: String = String::from("");
        while lookup.len() <= code_len as usize {
            lookup.push(Vec::new());
        }

        for _i in 0..code_len {
            let curr_val = code_num & 1;
            code_num >>= 1;
            if curr_val == 1 {
                code = format!("1{}", code);
            } else {
                code = format!("0{}", code);
            }
        }
        lookup[code_len as usize].push((code, symbol));

        curr_byte = in_bytes_iter.next().unwrap();
        let mut peek = in_bytes_iter.clone().peekable();
        let next_byte = match peek.peek() {
            Some(p) => *p,
            None => 0,
        };
        if curr_byte == SEPARATOR_UPPER && next_byte == SEPARATOR_LOWER {
            in_bytes_iter.next();
            break;
        }
    }
    println!("Received code: {:?}", lookup);

    // actually read the text:
    let mut output: Vec<u8> = Vec::new();
    let mut curr_str: String = String::from("");
    let mut curr_byte_res = in_bytes_iter.next();
    while curr_byte_res.is_some() {
        curr_byte = curr_byte_res.unwrap();
        for i in 0..8 {
            if (curr_byte >> (7 - i) & 1) == 1 {
                curr_str = format!("{}1", curr_str);
            } else {
                curr_str = format!("{}0", curr_str);
            }

            if lookup[curr_str.len()].len() != 0 {
                for s in &lookup[curr_str.len()] {
                    if curr_str.cmp(&s.0) == Ordering::Equal {
                        output.push(s.1);
                        curr_str = String::from("");
                    }
                }
            }
        }
        curr_byte_res = in_bytes_iter.next();
    }

    let write_res = fs::write(output_file, output);
    match write_res {
        Ok(_) => {}
        Err(e) => panic!("Decompression write error {}!", e),
    }
}
