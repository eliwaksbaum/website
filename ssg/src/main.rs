pub mod heml;
pub mod blog;

fn main()
{
    println!("=============");
    println!("Running SSG");
    
    println!("  Building Docs");
    heml::build("docs/", "../.com/");
    println!("  ..Done");

    println!("  Preparing Blog Previews");
    blog::prepare_previews("docs/public/blog/", "eags/blog-preview.heml", "../.com/assets/blog-previews.toml");
    println!("  ..Done");

    println!("..Done");
    println!("=============");
}