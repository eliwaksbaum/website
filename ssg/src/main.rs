use std::fs;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf, Display};

#[derive(Deserialize, Debug)]
struct InsertDef
{
    path: String,
    params: Vec<String>
}
type InsertCall = HashMap<String, String>;

fn main()
{
    let www_root = "https://eli.waksbaum.com/";
    let read_base = PathBuf::from("docs/");
    let write_base = PathBuf::from("../.com/public/");

    let file = fs::read_to_string("inserts/inserts.toml").expect("Unable to read inserts.toml");

    let inserts_table: HashMap<String, InsertDef> = toml::from_str(&file).expect("Unable to parse inserts.toml");

    copy_over(&read_base, &read_base, &write_base, &inserts_table);
    
}

fn copy_over(dir: &Path, read_base: &Path, write_base: &Path, inserts_table: &HashMap<String, InsertDef>) 
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
            copy_over(&path, read_base, write_base, &inserts_table);
        }
        else
        {
            let raw_page = fs::read_to_string(&path).expect(&format!("Could not read {}.", path.display()));
            let content = extract_page(raw_page, &inserts_table, &path.display(), None);
            fs::write(&write, content).expect(&format!("Could not write to {}", write.display()));
        }
    }
}

fn extract_page(raw_page: String, inserts_table: &HashMap<String, InsertDef>, file: &Display, prev: Option<&str>) -> String
{
    match (raw_page.find("<insert>"), raw_page.find("</insert>"))
    {
        (Some(start), Some(end)) => return replace_in_page(&raw_page, start, end, &inserts_table, file, prev),
        (None, None) => return raw_page,
        _ => {
            println!("Mismatched <insert> tags in {}. Continuing with parse.", file);
            return raw_page;
        }
    }
}

fn replace_in_page(page: &str, start: usize, end: usize, inserts_table: &HashMap<String, InsertDef>, file: &Display, prev: Option<&str>) -> String
{
    let call: InsertCall = toml::from_str(&page[start+8..end]).expect(&format!("Invalid toml in {}.", file));

    let insert_name = call.get("name").expect(&format!("No \"name\" key in an insert tag in {}.", file));
    match prev
    {
        Some(prev_name) => {
            if insert_name == prev_name
            {
                panic!("Illegal self-referential insert detected in the insert \"{}\".", insert_name);
            }
        }
        None => {}
    }
    let insert = inserts_table.get(insert_name).expect(&format!("No insert with the name \"{}\" exists.", insert_name));

    let replacements = insert.params.iter()
        .map(|p| (paramify(p), call.get(p).expect(&format!("An insert tag with the name set to \"{}\" is missing the required \"{}\" parameter in {}.", insert_name, p, file))))
        .collect::<HashMap<String, &String>>();

    let mut page_text = page.to_string();

    let insert_path = String::from("inserts/") + &insert.path;
    let insert_text = fs::read_to_string(&insert_path).expect(&format!("Could not open {}.", insert.path));
    let tag_space = &page_text[start..end+8];
    page_text = page_text.replace(tag_space, &insert_text);

    for (param, input) in replacements
    {
        page_text = page_text.replace(&param, input);
    }
    
    return extract_page(page_text, inserts_table, file, Some(&insert_name));
}

fn paramify(p: &str) -> String
{
    String::from("%@%") + p + "%@%"
}