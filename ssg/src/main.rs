use std::fmt::write;
use std::fs;
use std::ptr::read;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Debug)]
struct InsertDef
{
    path: String,
    params: Vec<String>
}
type InsertCall<'a> = HashMap<&'a str, &'a str>;

fn main()
{
    let www_root = "https://eli.waksbaum.com/";
    let read_base = PathBuf::from("docs/");
    let write_base = PathBuf::from("../.com/public/");

    let file = fs::read_to_string("inserts/inserts.toml").expect("Unable to read inserts.toml");

    let inserts_table: HashMap<&str, InsertDef> = toml::from_str(&file).expect("Unable to parse inserts.toml");

    copy_over(&read_base, &read_base, &write_base);
    
}

fn copy_over(dir: &Path, read_base: &Path, write_base: &Path) 
{
    let files = fs::read_dir(dir).expect(&format!("Could not open {}", dir.display()));
    for entry in files
    {
        let path = entry.unwrap().path();
        let write = write_base.join(path.strip_prefix(read_base).unwrap());
        if path.is_dir()
        {
            if !write.exists()
            {
                fs::create_dir(&write).expect(&format!("Could not create dir {}", write.display()));
            }
            copy_over(&path, read_base, write_base);
        }
        else
        {
            let content = fs::read_to_string(&path).unwrap();
            fs::write(&write, content).expect(&format!("Could not write to {}", write.display()));
        }
    }
}
