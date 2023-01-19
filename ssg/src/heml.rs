use std::fs;
use std::ops::Range;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use serde::Deserialize;

type EagTable = HashMap<String, Eag>;
#[derive(Deserialize)]
pub struct Eag
{
    pub path: String,
    pub text_params: Option<Vec<String>>,
    pub file_params: Option<Vec<String>>,
    pub list_params: Option<Vec<ListParam>>
}
#[derive(Deserialize)]
pub struct ListParam
{
    pub name: String,
    pub wrapper: String,
    pub join: String
}

pub type EagCall = HashMap<String, EagArg>;
#[derive(Deserialize)]
#[serde(untagged)]
pub enum EagArg
{
    Text(String),
    List(Vec<String>)
}
type EagRealization = HashMap<String, String>;

type FileCache= HashMap<String, String>;

pub struct Analysis<'a>
{
    pub range: Range<usize>,
    pub inside_text: &'a str,
    pub eag_call: EagCall,
    pub eag_name: &'a str
}

pub fn build(read_dir: &str, write_dir: &str)
{
    let table: EagTable = toml::from_str(&fs::read_to_string("eags/eags.toml").unwrap()).unwrap();
    let mut cache = FileCache::new();
    
    let read_base = PathBuf::from(read_dir);
    let write_base = PathBuf::from(write_dir);

    build_dir(&read_base, &read_base, &write_base, &table, &mut cache);
}

fn build_dir(dir: &Path, read_base: &Path, write_base: &Path, table: &EagTable, cache: &mut FileCache)
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
            build_dir(&path, read_base, &write_base, table, cache);
        }
        else
        {
            let file_write = write.with_extension("html");
            fs::write(&file_write, generate_page(&path, table, cache)).unwrap_or_else(|_| panic!("Could not write to {:?}", &file_write));
        }
    }
}

pub fn generate_page(read_path: &Path, table: &EagTable, cache: &mut FileCache) -> String
{
    let heml = fs::read_to_string(read_path).unwrap_or_else(|_| panic!("Could not open {:?}.", &read_path));

    convert(heml, table, cache).unwrap_or_else(|m| panic!("Conversion of {:?} failed while {}", read_path, m))
}

fn convert(heml: String, table: &EagTable, cache: &mut FileCache) -> Result<String, String>
{
    if heml.find("<<").is_none()
    {
        return Ok(heml);
    }

    let analysis = match parse(&heml) {
        Ok(a) => a,
        Err(e) => { return Err(e); }
    };

    let eag = match table.get(analysis.eag_name) {
        Some(e) => e,
        None => { return Err(String::from("looking for <<") + analysis.eag_name + ">> in the eag table. Are you sure it's spelled right?") }
    };

    let rlz = match generate_eag_realization(eag, &analysis.eag_call, analysis.inside_text, cache) {
        Ok(r) => r,
        Err(param) => { return Err(String::from("generating a realization of <<") + analysis.eag_name + ">>. Required argument \"" + param + "\" is missing or misformatted.") }
    };

    let eag_doc = cache.entry(eag.path.clone()).or_insert_with(|| read_eag_doc(&eag.path));

    let replacement = replace(&eag_doc, &rlz);

    let mut heml = heml.clone();
    heml.replace_range(analysis.range, &replacement);

    convert(heml, table, cache)
}

pub fn parse<'a>(heml: &'a str) -> Result<Analysis<'a>, String>
{
    let open_open = match heml.find("<<") {
        Some(i) => i,
        None => panic!() //should really be impossible
    };

    let open_close = match heml.find(">>") {
        Some(i) => i,
        None => { return Err(String::from("searching for a \">>\" to close the eag.")) }
    };

    let eag = &heml[(open_open + 2)..(open_close)];

    let toml_end = match (&heml[(open_close + 2)..]).find("<") {
        Some(i) => i + open_close + 2,
        None => { return Err(String::from("searching for a \"<\" to mark the end of <<") + eag + ">>'s arguments.") }
    };

    let close_open = match heml.find(&(String::from("<</") + eag + ">>")) {
        Some(i) => i,
        None => { return Err(String::from("searching for \"<</") + eag + ">>\" to close the section.") }
    };

    let close_close = close_open + eag.len() + 5;

    let call: EagCall = match toml::from_str(&heml[(open_close + 2)..toml_end]) {
        Ok(a) => a,
        Err(e) => { return Err(String::from("parsing <<") + eag + ">>'s arguments. Here's the toml error: " + &e.to_string()) }
    };

    Ok(Analysis {
        range: (open_open..close_close),
        inside_text: &heml[toml_end..close_open],
        eag_call: call,
        eag_name: eag
    })
}

pub fn generate_eag_realization<'a>(eag: &'a Eag, call: &EagCall, inside_text: &str, cache: &mut FileCache) -> Result<EagRealization, &'a str>
{
    let mut rlz = EagRealization::new();
    rlz.insert(String::from("{{inside}}"), inside_text.to_string());

    eag.text_params.as_ref()
        .map_or_else(|| Ok(()), |params| place_text_params_into_realization(&params, call, &mut rlz))
        .and_then(|_|
            eag.file_params.as_ref()
                .map_or_else(|| Ok(()), |params| place_file_params_into_realization(&params, call, &mut rlz, cache))
        )
        .and_then(|_|
            eag.list_params.as_ref()
                .map_or_else(|| Ok(()), |params| place_list_params_into_realization(&params, call, &mut rlz))
        )
        .map(|_| rlz)
}

fn place_text_params_into_realization<'a>(params: &'a Vec<String>, call: &EagCall, rlz: &mut EagRealization) -> Result<(), &'a str>
{
    for p in params
    {
        match call.get(p)
        {
            Some(EagArg::Text(arg)) => { rlz.insert(text_paramify(p), arg.clone()); }
            _ => { return Err(p); }
        }
    }
    Ok(())
}

fn text_paramify(p: &str) -> String
{
    String::from("{{") + p + "}}"
}

fn place_file_params_into_realization<'a>(params: &'a Vec<String>, call: &EagCall, rlz: &mut EagRealization, cache: &mut FileCache) -> Result<(), &'a str>
{
    for p in params
    {
        match call.get(p)
        {
            Some(EagArg::Text(path)) => {
                let file_text = cache.entry(path.clone()).or_insert_with(|| read_dump_doc(&path));
                rlz.insert(file_paramify(p), file_text.clone());
            },
            _ => { return Err(p); }
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
    String::from("@@") + p + "@@"
}

fn place_list_params_into_realization<'a>(params: &'a Vec<ListParam>, call: &EagCall, rlz: &mut EagRealization) -> Result<(), &'a str>
{
    for p in params
    {
        match call.get(&p.name)
        {
            Some(EagArg::List(items)) => { rlz.insert(list_paramify(&p.name), get_list_insert(p, items)); },
            _ => { return Err(&p.name); }
        }
    }
    Ok(())
}

fn get_list_insert(param: &ListParam, items: &Vec<String>) -> String
{
    items.iter()
        .map(|item| param.wrapper.replace(&text_paramify(&param.name), item))
        .reduce(|a, b| a + &param.join + &b)
        .unwrap_or_default()
}

fn list_paramify(p: &str) -> String
{
    String::from("[[") + p + "]]"
}

fn read_eag_doc(path: &str) -> String
{
    let eag_path = String::from("eags/") + path;
    return fs::read_to_string(&eag_path).unwrap_or_else(|_| panic!("Could not open {}.", &eag_path));
}

pub fn replace(page: &str, rlz: &EagRealization) -> String
{
    let mut next = page.to_string();
    for (param, arg) in rlz
    {
        next = next.replace(param, arg);
    }
    next
}