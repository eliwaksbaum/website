use std::fs;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf, Display};

#[derive(Deserialize)]
struct InsertDef
{
    path: String,
    text_params: Option<Vec<String>>,
    file_params: Option<Vec<String>>
}
type InsertCall = HashMap<String, String>;

struct InsertTables
{
    defs: HashMap<String, InsertDef>,
    texts: HashMap<String, String>
}

fn main()
{
    let www_root = "https://eli.waksbaum.com/";
    let read_base = PathBuf::from("docs/");
    let write_base = PathBuf::from("../.com/public/");

    let file = fs::read_to_string("inserts/inserts.toml").expect("Unable to read inserts.toml");

    let inserts_table: HashMap<String, InsertDef> = toml::from_str(&file).expect("Unable to parse inserts.toml");
    let mut tables = InsertTables{ defs: inserts_table, texts: HashMap::new() };

    copy_over(&read_base, &read_base, &write_base, &mut tables);
    
}

fn copy_over(dir: &Path, read_base: &Path, write_base: &Path, tables: &mut InsertTables) 
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
            copy_over(&path, read_base, write_base, tables);
        }
        else
        {
            let raw_page = fs::read_to_string(&path).expect(&format!("Could not read {}.", path.display()));
            let content = extract_page(raw_page, tables, &path.display(), None);
            fs::write(&write, content).expect(&format!("Could not write to {}", write.display()));
        }
    }
}

fn extract_page(raw_page: String, tables: &mut InsertTables, file: &Display, prev: Option<&str>) -> String
{
    match (raw_page.find("<insert>"), raw_page.find("</insert>"))
    {
        (Some(start), Some(end)) => return replace_insert(&raw_page, start, end, tables, file, prev),
        (None, None) => return raw_page,
        _ =>
        {
            println!("Mismatched <insert> tags in {}. Continuing with parse.", file);
            return raw_page;
        }
    }
}

fn replace_insert(page: &str, start: usize, end: usize, tables: &mut InsertTables, file: &Display, prev: Option<&str>) -> String
{
    let call: InsertCall = toml::from_str(&page[start+8..end]).expect(&format!("Invalid toml while parsing {}: {}", file, &page[start+8..end]));

    let insert_name = call.get("name").expect(&format!("No \"name\" key in an insert tag in {}.", file));
    // match prev
    // {
    //     Some(prev_name) => {
    //         if insert_name == prev_name
    //         {
    //             panic!("Illegal self-referential insert detected in the insert \"{}\".", insert_name);
    //         }
    //     }
    //     None => {}
    // }

    let defs = &tables.defs;
    let insert = defs.get(insert_name).expect(&format!("No insert with the name \"{}\" exists.", insert_name));

    let mut page_text = page.to_string();
    
    let insert_text = get_or_put_read(&insert.path, &mut tables.texts);
    let tag_space = &page_text[start..end+9];
    page_text = page_text.replace(tag_space, &insert_text);

    match &insert.text_params
    {
        Some(params) => { page_text = replace_text_params(&page_text, &params, &call, &insert_name, file); },
        None => {}    
    }

    match &insert.file_params
    {
        Some(params) => { page_text = replace_file_params(&page_text, &params, &call, &mut tables.texts, &insert_name, file); },
        None => {}
    }
    
    return extract_page(page_text, tables, file, Some(&insert_name));
}

fn replace_text_params(page: &str, params: &Vec<String>, call: &InsertCall, name: &str, file: &Display) -> String
{

    let mut parsed = page.to_string();

    let replacements = params.iter()
        .map(|p| (paramify(p), call.get(p).expect(&format!("An insert tag with the name set to \"{}\" is missing the required \"{}\" parameter in {}.", name, p, file))))
        .collect::<HashMap<String, &String>>();    

    for (param, arg) in replacements
    {
        parsed = parsed.replace(&param, arg);
    }
    return parsed;
}

fn replace_file_params(page: &str, params: &Vec<String>, call: &InsertCall, path_table: &mut HashMap<String, String>, name: &str, file: &Display) -> String
{
    let mut parsed = page.to_string();

    let replacements = params.iter()
        .map(|p| (paramify(p), call.get(p).expect(&format!("An insert tag with the name set to \"{}\" is missing the required \"{}\" parameter in {}.", name, p, file))))
        .map(|(param, path)| (param, get_or_put_read(&(String::from("nullaries/") + path), path_table)))
        .collect::<HashMap<String, String>>();    

    for (param, file_text) in replacements
    {
        parsed = parsed.replace(&param, &file_text);
    }
    return parsed;
}

fn get_or_put_read(path: &str, table: &mut HashMap<String, String>) -> String
{
    table.entry(path.to_string()).or_insert_with(|| read_insert(&path)).to_string()
}

fn read_insert(path: &str) -> String
{
    let insert_path = String::from("inserts/") + path;
    return fs::read_to_string(&insert_path).expect(&format!("Could not open {}.", &insert_path));
}

fn paramify(p: &str) -> String
{
    String::from("%@%") + p + "%@%"
}