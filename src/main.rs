use std::io::Write;
use std::io::Read;
use clap::*;
use directories::BaseDirs;
use std::path::Path;
use std::fs::File;
use std::*;


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
                make_error("Error making flex config: ", why);
            }
            if let Err(why) = File::create(&[base_dirs.config_dir().to_str().unwrap(), "/flex/flex.json"].join("")) {
                make_error("Error making flex config: ", why);
            }
            if let Err(why) = write_file([base_dirs.config_dir().to_str().unwrap(), "/flex/flex.json"].join(""), "{\n}".to_string()) {
                make_error("Error writing to flex config: ", why);
            }
        }
        if matches.is_present("add") {
            let mut file = File::open(&[base_dirs.config_dir().to_str().unwrap(), "/flex/flex.json"].join("")).unwrap();
            let mut data = String::new();
            file.read_to_string(&mut data).unwrap();
            add_item_to_json(
                input("What is the imdb url (ex https://www.imdb.com/title/tt10838180/)? : ").trim().to_string().replace("/", "").split("tt").last().expect("The url was not formated right").to_string(),
                input("Where is the movie location on your computer : ").trim().to_string(),
                [base_dirs.config_dir().to_str().unwrap(), "/flex/flex.json"].join("")
            );
            println!("Your movie was added!");
        }
    }
}

fn write_file(file:String, text:String) -> io::Result<()> {
    let mut file = File::create(file)?;
    file.write_all(text.trim().as_ref())?;
    Ok(())
}

fn make_error(beginning_text:&str, why:io::Error) {
    println!("{}{:?}", beginning_text, why);
    process::exit(1);
}

fn input(message:&str) -> String
{
    print!("{}", message);
    io::stdout().flush().expect("flush failed!");
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn add_item_to_json(movie_id:String, location_on_computer:String, location_of_json:String) {
    let file_cont = fs::read_to_string(location_of_json.clone()).expect("Unable to read file");
    let mut write:String = file_cont.chars().collect::<Vec<char>>()[..file_cont.len()-2].into_iter().collect();
    if file_cont.chars().nth(file_cont.len()-3).unwrap() == '\"' {
        write.push(',');
    }
    write.push_str(format!("\n\t\"{}\":\"{}\"\n", movie_id, location_on_computer).as_str());
    write.push('}');
    if let Err(why) = write_file(location_of_json, write) {
        make_error("Error writing to flex config: ", why);
    }
}