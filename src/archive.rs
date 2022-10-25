use std::cmp::Ordering;
use std::fs;
use std::path::Path;

pub fn create_archive(files: Vec<String>) -> Vec<u8> {
    let mut ret_vec: Vec<u8> = Vec::new();
    ret_vec.extend([
        'h' as u8, 'f' as u8, 'm' as u8, 'a' as u8, 'r' as u8, 'c' as u8,
    ]);
    ret_vec.extend(files.len().to_be_bytes());
    for f in files {
        let fpath = Path::new(&f);
        let f_bytes = if fpath.is_dir() {
            let dir_res = fs::read_dir(f.clone()).unwrap();
            let mut file_vec: Vec<String> = Vec::new();
            for i in dir_res {
                match i {
                    Ok(d) => file_vec.push(String::from(d.path().to_str().unwrap())),
                    Err(e) => println!("Dir entry error {:?}!", e),
                }
            }
            create_archive(file_vec)
        } else {
            fs::read(f.clone()).unwrap()
        };

        let fname = fpath.file_name().unwrap();
        ret_vec.push(fname.len() as u8);
        ret_vec.append(&mut Vec::from(fname.to_str().unwrap().as_bytes()));
        if !fpath.is_dir() {
            ret_vec.push('f' as u8);
        } else {
            ret_vec.push('d' as u8);
        }
        let f_len = f_bytes.len();
        ret_vec.extend(f_len.to_be_bytes());
        ret_vec.extend(f_bytes);
    }

    ret_vec
}

pub fn write_archive(archive: Vec<u8>, dirname: Option<String>) {
    if archive[0..6].cmp(&[
        'h' as u8, 'f' as u8, 'm' as u8, 'a' as u8, 'r' as u8, 'c' as u8,
    ]) != Ordering::Equal
    {
        panic!("Not a valid archive!");
    }

    let num_files = usize::from_be_bytes(archive[6..14].try_into().unwrap());
    let mut curr_pos = 14;

    if dirname.is_some() {
        let dir = dirname.clone().unwrap();
        if !Path::new(&dir).is_dir() {
            // create new directory
            let cd_res = fs::create_dir(dir.clone());
            match cd_res {
                Ok(_) => {}
                Err(e) => {
                    panic!("Failed to create directory {} because of error {}!", dir, e)
                }
            }
        }
    }

    for _i in 0..num_files {
        let fname_len = archive[curr_pos] as usize;
        curr_pos += 1;
        let fname: String = archive[curr_pos..(curr_pos + fname_len)]
            .to_vec()
            .into_iter()
            .map(|x| x as char)
            .collect();
        curr_pos += fname_len;
        let f_type = archive[curr_pos] as char;
        curr_pos += 1;

        let f_len = usize::from_be_bytes(archive[curr_pos..(curr_pos + 8)].try_into().unwrap());
        curr_pos += 8;
        let f_bytes = archive[curr_pos..(curr_pos + f_len)].to_vec();

        if f_type == 'd' {
            let dir_path = match &dirname {
                Some(d) => format!("{}/{}", d, fname),
                None => fname,
            };
            let mkdir_res = fs::create_dir(dir_path.clone());
            match mkdir_res {
                Ok(_) => {}
                Err(e) => {
                    panic!("Could not make directory {} error {}!", dir_path, e);
                }
            }
            write_archive(f_bytes, Some(dir_path));
        } else {
            let out_file = match &dirname {
                Some(d) => format!("{}/{}", d, fname),
                None => format!("{}", fname),
            };
            let write_res = fs::write(out_file.clone(), f_bytes);
            match write_res {
                Ok(_) => {}
                Err(e) => panic!("Could not make file {} error {}!", out_file, e),
            }
        }
        curr_pos += f_len;
    }
}
