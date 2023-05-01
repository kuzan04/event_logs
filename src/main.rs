use std::io::{Result, stdin, stdout, Write};
use std::fs;

mod handle;

fn get_path() -> Result<String> {
    loop {
        let mut path = String::new();
        print!("Typing the directory at you select: ");
        stdout().flush().unwrap();
        stdin()
            .read_line(&mut path)
            .expect("Failed to read input.");
        if path.trim().contains('/') {
            break Ok(path)
        }else{
            println!("Invaild input, please try again.");
            continue;
        }
    }
}

fn set_path(mut p: String) -> Result<String> {
    if p.trim().chars().nth(p.trim().chars().count() - 1).unwrap() != '/' {
        p = p.trim().to_owned() + "/";
    }
    loop {
        match fs::metadata(p.trim()){
            Ok(ref pt) if pt.is_dir() => {
                break Ok(p);
            }
            Ok(_) => todo!(),
            Err(_) => {
                fs::create_dir_all(p.trim())?;
                continue;
            },
        };
    }
}

fn main() -> Result<()> {
    let path = set_path(get_path()?)?;
    handle::dump(path)?;
    Ok(())
}
