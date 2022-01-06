use std::io::Write;
use clap::*;
use directories::BaseDirs;
use std::path::Path;
use std::fs::File;
use std::fs;


fn main() {
    let matches = App::new("Flex")
        .version("0.01")
        .about("A movie sharing platform")
        .arg(
            Arg::with_name("add")
                .short("a")
                .long("add")
                .help("add a movie")
                .takes_value(false),
        )
        .get_matches();
    if let Some(base_dirs) = BaseDirs::new() {
        if !Path::new(&[base_dirs.config_dir().to_str().unwrap(), "/flex/flex.json"].join("")).exists() {
            if let Err(why) = fs::create_dir(&[base_dirs.config_dir().to_str().unwrap(), "/flex"].join("")) {
                println!("Error making flex config: {:?}", why);
                std::process::exit(1);
            }
            if let Err(why) = File::create(&[base_dirs.config_dir().to_str().unwrap(), "/flex/flex.json"].join("")) {
                println!("Error making flex config: {:?}", why);
                std::process::exit(1);
            }
            if let Err(why) = write_file([base_dirs.config_dir().to_str().unwrap(), "/flex/flex.json"].join(""), "{}".to_string()) {
                println!("Error writing to flex config: {:?}", why);
                std::process::exit(1);
            }
        }
    }
    if matches.is_present("add") {
        
    }
}

fn write_file(file:String, text:String) -> std::io::Result<()> {
    let mut file = File::create(file)?;
    file.write_all(text.trim().as_ref())?;
    Ok(())
}