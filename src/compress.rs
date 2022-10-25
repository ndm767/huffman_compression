use crate::archive::create_archive;
use crate::arg_handler::Args;
use crate::binary_tree::BinaryNode;
use crate::consts::{SEPARATOR_LOWER, SEPARATOR_UPPER};

use std::fs;
use std::path::Path;

fn find_probs(bytes: Vec<u8>) -> Vec<(u8, f32)> {
    let mut num_each: Vec<u32> = Vec::new();
    for _i in 0..=u8::MAX {
        num_each.push(0);
    }

    let mut total_chars: u32 = 0;
    for b in bytes.iter() {
        num_each[*b as usize] += 1;
        total_chars += 1;
    }

    let mut ret: Vec<(u8, f32)> = Vec::new();
    let tc_float: f32 = total_chars as f32;
    for val in num_each.iter().enumerate() {
        if *val.1 != 0 {
            ret.push((val.0 as u8, (*val.1 as f32) / tc_float));
        }
    }

    println!("total chars: {}", total_chars);

    return ret;
}

fn string_to_bin_u32(str: String) -> u32 {
    let mut ret: u32 = 0;
    for c in str.chars() {
        ret <<= 1;
        if c == '1' {
            ret += 1;
        }
    }
    ret
}

fn vec_insert_val(v: &mut Vec<BinaryNode>, val: BinaryNode) {
    if v.len() == 0 {
        v.push(val);
        return;
    }
    for i in 0..v.len() {
        if v[i].get_prob() > val.get_prob() {
            v.insert(i, val);
            return;
        }
    }
    v.push(val);
}

pub fn compress(args: Args) {
    let input_file = args.input_file;
    let output_file = match args.output_file {
        Some(s) => s,
        None => String::from("out.hfm"),
    };

    println!(
        "Compressing input_file {} to output_file {}",
        input_file, output_file
    );

    // read file and get frequencies
    let in_bytes = if args.is_dir {
        let dir_res = fs::read_dir(input_file.clone()).unwrap();
        let mut file_vec: Vec<String> = Vec::new();
        for i in dir_res {
            match i {
                Ok(d) => file_vec.push(String::from(d.path().to_str().unwrap())),
                Err(e) => println!("Dir entry error {:?}!", e),
            }
        }
        create_archive(file_vec)
    } else {
        fs::read(input_file.clone()).unwrap()
    };
    let in_size = in_bytes.len();
    println!("input size: {}", in_size);

    let mut alphabet = find_probs(in_bytes.clone());
    alphabet.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let mut tree: Vec<BinaryNode> = vec![];
    for a in alphabet.iter() {
        tree.push(BinaryNode::new(a.1, Some(a.0), None, None));
    }

    // generate huffman code
    while tree.len() > 1 {
        let n1 = tree[0].clone();
        let n2 = tree[1].clone();
        tree.remove(0);
        tree.remove(0);
        vec_insert_val(
            &mut tree,
            BinaryNode::new(
                n1.get_prob() + n2.get_prob(),
                None,
                Some(Box::new(n1)),
                Some(Box::new(n2)),
            ),
        );
    }

    let mut huffman_code = tree[0].get_huffman_code();
    huffman_code.sort_by(|a, b| a.0.cmp(&b.0));
    println!("Generated code: {:?}", huffman_code);

    let csize = tree[0].get_comp_size(alphabet, in_size);
    println!(
        "Original size: {}, compressed size: {}, ratio: {}",
        in_size,
        csize.ceil(),
        csize.ceil() / (in_size as f32)
    );

    // write code to file for decompression
    let mut output: Vec<u8> = Vec::new();
    let input_path = Path::new(&input_file);
    if args.is_dir {
        output.append(&mut Vec::from(input_file.as_bytes()))
    } else {
        output.append(&mut Vec::from(
            format!(
                "{}{}",
                input_path.file_stem().unwrap().to_str().unwrap(),
                match input_path.extension() {
                    Some(s) => format!(".{}", s.to_str().unwrap()),
                    None => String::from(""),
                }
            )
            .as_bytes(),
        ));
    }
    output.push(SEPARATOR_UPPER);
    output.push(SEPARATOR_LOWER);
    for c in huffman_code.iter() {
        output.push(c.0);
        output.push(c.1.len() as u8);
        output.extend_from_slice(&string_to_bin_u32(c.1.clone()).to_be_bytes());
    }
    output.push(SEPARATOR_UPPER);
    output.push(SEPARATOR_LOWER);

    let out_start_len = output.len();

    //create lookup table
    let mut lookup: Vec<String> = Vec::new();
    for c in huffman_code {
        while c.0 as usize >= lookup.len() {
            lookup.push(String::from(""));
        }

        lookup[c.0 as usize] = c.1;
    }
    // actually compress the file
    let mut byte_pos: usize = 0;
    let mut curr_byte: u8 = 0;
    for i in in_bytes {
        let curr_code = lookup[i as usize].clone();
        for c in curr_code.chars() {
            curr_byte <<= 1;
            byte_pos += 1;
            if c == '1' {
                curr_byte += 1;
            }

            if byte_pos == 8 {
                byte_pos = 0;
                output.push(curr_byte);
                curr_byte = 0;
            }
        }
    }
    if byte_pos != 0 {
        curr_byte <<= 8 - byte_pos;
        output.push(curr_byte);
        output.push(byte_pos as u8);
    }

    println!(
        "Output size: {} ({} without header)",
        output.len(),
        output.len() - out_start_len
    );

    let write_res = fs::write(output_file, output);
    match write_res {
        Ok(_o) => {}
        Err(e) => {
            panic!("Could not write to output file! Error: {:?}", e);
        }
    }
}
