use std::{env, fs};


pub fn get_current_directory() -> std::io::Result<fs::ReadDir> {
    let ret = env::current_dir()?;
    if !ret.is_dir() {
        panic!("Provided path is not a directory");
    }
    fs::read_dir(ret)
}

pub fn list_cmd(path: &String)
{
   match fs::metadata(path) {
        Ok(md) => {
            if !md.is_dir() {
                println!("Secified path does not point to a directory.");
                return;
            }
            let files = fs::read_dir(path).unwrap();
            for (index, file) in files.enumerate() {
                println!("{}: {}", index, file.unwrap().file_name().into_string().unwrap());
            }
        },
        Err(_) => {},
    }
}
