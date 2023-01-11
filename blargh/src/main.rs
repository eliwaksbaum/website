use quick_xml::de;
use serde::Deserialize;
use std::fs;

#[derive(Desiralize)]
struct Doc
{

}

fn main()
{
    let foo = fs::read_to_string("foo.xml").unwrap();
}
