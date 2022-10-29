use crate::archive::write_archive;
use crate::arg_handler::Args;
use crate::consts::{SEPARATOR_LOWER, SEPARATOR_UPPER};

use std::{cmp::Ordering, fs};

pub fn decompress(args: Args) {
    let input_file = args.input_file;

    let mut in_bytes = fs::read(input_file.clone()).unwrap();
    let in_len = in_bytes.len();
    println!("input len: {}", in_len);
    let last_byte = *in_bytes.last().clone().unwrap();
    in_bytes.remove(in_bytes.len() - 1);

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

    // get encoding
    let mut lookup: Vec<Vec<i16>> = Vec::new();
    loop {
        let symbol = curr_byte;
        let code_len = in_bytes_iter.next().unwrap();
        let code_entries: [u8; 4] = [
            in_bytes_iter.next().unwrap(),
            in_bytes_iter.next().unwrap(),
            in_bytes_iter.next().unwrap(),
            in_bytes_iter.next().unwrap(),
        ];
        let code_num = u32::from_be_bytes(code_entries);

        while lookup.len() <= code_len as usize {
            lookup.push(Vec::new());
        }
        while lookup[code_len as usize].len() <= code_num as usize {
            lookup[code_len as usize].push(-1);
        }

        lookup[code_len as usize][code_num as usize] = symbol as i16;

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

    // actually read the text:
    let mut output: Vec<u8> = Vec::new();
    let mut curr_symb: u32 = 0;
    let mut curr_symb_len: u8 = 0;
    let mut curr_byte_res = in_bytes_iter.next();
    while curr_byte_res.is_some() {
        curr_byte = curr_byte_res.unwrap();
        curr_byte_res = in_bytes_iter.next();
        let end = if curr_byte_res.is_none() && last_byte != 0 {
            last_byte
        } else {
            8
        };
        for i in 0..end {
            curr_symb <<= 1;
            curr_symb_len += 1;
            if (curr_byte >> (7 - i) & 1) == 1 {
                curr_symb += 1;
            }

            if (curr_symb_len as usize) < lookup.len() {
                let curr_lookup_vec = &lookup[curr_symb_len as usize];
                if (curr_symb as usize) < curr_lookup_vec.len() {
                    let curr_lookup = curr_lookup_vec[curr_symb as usize];
                    if curr_lookup != -1 {
                        output.push(curr_lookup as u8);
                        curr_symb = 0;
                        curr_symb_len = 0;
                    }
                }
            } else {
                panic!("Code {:#16b}/{} is not in lookup!", curr_symb, curr_symb);
            }
        }
    }

    if output[0..6].cmp(&[
        'h' as u8, 'f' as u8, 'm' as u8, 'a' as u8, 'r' as u8, 'c' as u8,
    ]) == Ordering::Equal
    {
        write_archive(output, args.output_file);
    } else {
        let output_file = match args.output_file {
            Some(s) => s,
            None => orig_file_name,
        };
        let write_res = fs::write(output_file, output);
        match write_res {
            Ok(_) => {}
            Err(e) => panic!("Decompression write error {}!", e),
        }
    }
}
