use std::fs;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

type EagTable = HashMap<String, Eag>;
#[derive(Deserialize)]
struct Eag
{
    path: String,
    text_params: Option<Vec<String>>,
    file_params: Option<Vec<String>>
}
type EagRealization = HashMap<String, String>;
type EagArgs = HashMap<String, String>;
type FileCache= HashMap<String, String>;

fn main()
{
    let table: EagTable = toml::from_str(&fs::read_to_string("eags/eags.toml").unwrap()).unwrap();
    let mut cache = FileCache::new();
    
    let read_base = PathBuf::from("docs/");
    let write_base = PathBuf::from("../.com/");

    build(&read_base, &read_base, &write_base, &table, &mut cache);
}

fn build(dir: &Path, read_base: &Path, write_base: &Path, table: &EagTable, cache: &mut FileCache)
{
    let files = fs::read_dir(dir).unwrap_or_else(|_| panic!("Could not open {:?}.", &dir));
    for entry in files
    {
        let path = entry.expect("Error in the DirEntry iterator while building.").path();
        let write = write_base.join(path.strip_prefix(read_base).unwrap());
        if path.is_dir()
        {
            if !write.exists()
            {
                fs::create_dir(&write).unwrap_or_else(|_| panic!("Could not create dir {}", write.display()));
            }
            build(&path, read_base, &write_base, table, cache);
        }
        else
        {
            let file_write = write.with_extension("html");
            fs::write(&file_write, generate_page(&path, table, cache)).unwrap_or_else(|_| panic!("Could not write to {:?}", &file_write));
        }
    }
}

fn generate_page(read_path: &Path, table: &EagTable, cache: &mut FileCache) -> String
{
    let heml = fs::read_to_string(read_path).unwrap_or_else(|_| panic!("Could not open {:?}.", &read_path));
    
    // if &heml[0..2] != "<<"
    // {
    //     panic!("Failed to parse {:?}. HEML files must start with a double-angle-bracketed eag.", read_path);
    // }

    parse(heml, table, cache).unwrap_or_else(|m| panic!("Parsing of {:?} failed while {}", read_path, m))
}

fn parse(mut heml: String, table: &EagTable, cache: &mut FileCache) -> Result<String, String>
{
    let open_open = match heml.find("<<") {
        Some(i) => i,
        None => { return Ok(heml) }
    };

    let open_close = match heml.find(">>") {
        Some(i) => i,
        None => { return Err(String::from("searching for a \">>\" to close the eag.")) }
    };

    let eag_name = &heml[(open_open + 2)..(open_close)];

    let toml_end = match (&heml[(open_close + 2)..]).find("<") {
        Some(i) => i + open_close + 2,
        None => { return Err(String::from("searching for a \"<\" to mark the end of <<") + eag_name + ">>'s arguments.") }
    };

    let close_open = match heml.find(&(String::from("<</") + eag_name + ">>")) {
        Some(i) => i,
        None => { return Err(String::from("searching for \"<</") + eag_name + ">>\" to close the section.") }
    };

    let close_close = close_open + eag_name.len() + 5;

    let args: EagArgs = match toml::from_str(&heml[(open_close + 2)..toml_end]) {
        Ok(a) => a,
        Err(e) => { return Err(String::from("Parsing <<") + eag_name + ">>'s arguments. Here's the toml error: " + &e.to_string()) }
    };

    let eag = match table.get(eag_name) {
        Some(e) => e,
        None => { return Err(String::from("Looking for <<") + eag_name + ">> in the eag table. Are you sure it's spelled right?") }
    };

    let rlz = match generate_eag_realization(eag, &args, &heml[toml_end..close_open], cache) {
        Ok(r) => r,
        Err(param) => { return Err(String::from("Generating a realization of <<") + eag_name + ">>. Could not find required argument " + param + ".") }
    };

    let eag_doc = read_eag_doc(&eag.path);

    let replacement = replace(&eag_doc, &rlz);

    heml.replace_range(open_open..close_close, &replacement);

    parse(heml, table, cache)
}

fn generate_eag_realization<'a>(eag: &'a Eag, args: &EagArgs, inside_text: &str, cache: &mut FileCache) -> Result<EagRealization, &'a str>
{
    let mut rlz = EagRealization::new();

    let text_result = match &eag.text_params
    {
        Some(params) => place_text_params_into_realization(&params, args, &mut rlz),
        None => Ok(())
    };

    let file_result = match &eag.file_params
    {
        Some(params) => place_file_params_into_realization(&params, args, &mut rlz, cache),
        None => Ok(())
    };

    rlz.insert(String::from("%@%inside%@%"), inside_text.to_string());

    text_result.and(file_result).map(|_| rlz)
}

fn place_text_params_into_realization<'a>(params: &'a Vec<String>, args: &EagArgs, rlz: &mut EagRealization) -> Result<(), &'a str>
{
    for p in params
    {
        match args.get(p)
        {
            Some(arg) => { rlz.insert(text_paramify(p), arg.clone()); }
            None => { return Err(p); }
        }
    }
    Ok(())
}

fn text_paramify(p: &str) -> String
{
    String::from("%@%") + p + "%@%"
}

fn place_file_params_into_realization<'a>(params: &'a Vec<String>, args: &EagArgs, rlz: &mut EagRealization, cache: &mut FileCache) -> Result<(), &'a str>
{
    for p in params
    {
        match args.get(p)
        {
            Some(path) => {
                let file_text = cache.entry(path.clone()).or_insert_with(|| read_dump_doc(&path));
                rlz.insert(file_paramify(p), file_text.clone());
            },
            None => { return Err(p); }
        }
    }
    Ok(())
}

fn read_dump_doc(path: &str) -> String
{
    let dump_path = String::from("dumps/") + path;
    return fs::read_to_string(&dump_path).unwrap_or_else(|_| panic!("Could not open {}.", &dump_path));
}

fn file_paramify(p: &str) -> String
{
    String::from("@%@") + p + "@%@"
}

fn read_eag_doc(path: &str) -> String
{
    let eag_path = String::from("eags/") + path;
    return fs::read_to_string(&eag_path).unwrap_or_else(|_| panic!("Could not open {}.", &eag_path));
}

fn replace(page: &str, rlz: &EagRealization) -> String
{
    let mut next = page.to_string();
    for (param, arg) in rlz
    {
        next = next.replace(param, arg);
    }
    next
}