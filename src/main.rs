use clap::*;
use directories::BaseDirs;
use serde_json::Value;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::path::Path;
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
        if !Path::new(&[base_dirs.config_dir().to_str().unwrap(), "/flex/flex.json"].join(""))
            .exists()
        {
            if let Err(why) =
                fs::create_dir(&[base_dirs.config_dir().to_str().unwrap(), "/flex"].join(""))
            {
                make_error("Error making flex config: ", why);
            }
            if let Err(why) = File::create(
                &[base_dirs.config_dir().to_str().unwrap(), "/flex/flex.json"].join(""),
            ) {
                make_error("Error making flex config: ", why);
            }
            if let Err(why) = write_file(
                [base_dirs.config_dir().to_str().unwrap(), "/flex/flex.json"].join(""),
                "{\n}".to_string(),
            ) {
                make_error("Error writing to flex config: ", why);
            }
        }
        if matches.is_present("add") {
            add_item_to_json(
                input("What is the imdb url (ex https://www.imdb.com/title/tt10838180/)? : ")
                    .trim()
                    .to_string()
                    .replace("/", "")
                    .split("tt")
                    .last()
                    .expect("The url was not formated right")
                    .to_string(),
                input("Where is the movie location on your computer : ")
                    .trim()
                    .to_string(),
                [base_dirs.config_dir().to_str().unwrap(), "/flex/flex.json"].join(""),
            );
            println!("Your movie was added!");
        } else {
            let listener = TcpListener::bind("0.0.0.0:80").unwrap();
            for stream in listener.incoming() {
                thread::spawn(move || {
                    let mut stream = stream.unwrap();
                    let mut buffer = [0; 4096];
                    stream.read(&mut buffer).unwrap();
                    let response: String = String::from_utf8_lossy(&buffer).to_string();
                    let mut range: (usize, usize) = (0, 0);
                    for i in response.to_lowercase().lines() {
                        if i.starts_with("range: ") {
                            println!("{}", i);
                            if i.split("-").last().unwrap().len() > 0 {
                                range = (
                                    i.split("bytes=")
                                        .last()
                                        .unwrap()
                                        .split("-")
                                        .nth(0)
                                        .unwrap()
                                        .parse()
                                        .unwrap(),
                                    i.split("bytes=")
                                        .last()
                                        .unwrap()
                                        .split("-")
                                        .last()
                                        .unwrap()
                                        .parse()
                                        .unwrap(),
                                );
                            } else {
                                range = (
                                    i.split("bytes=")
                                        .last()
                                        .unwrap()
                                        .split("-")
                                        .nth(0)
                                        .unwrap()
                                        .parse()
                                        .unwrap(),
                                    0,
                                );
                            }
                        }
                    }
                    if response.split(' ').count() > 1 {
                        let wants = response.split(' ').nth(1).unwrap();
                        let file_wants = match wants {
                            "/" => "index.html".to_string(),
                            "/index.html" => "index.html".to_string(),
                            "/style.css" => "style.css".to_string(),
                            "/favicon.ico" => "favicon.ico".to_string(),
                            "/main.js" => "main.js".to_string(),
                            _ => {
                                if let Some(base_dirs) = BaseDirs::new() {
                                    let mut file = File::open(
                                        &[
                                            base_dirs.config_dir().to_str().unwrap(),
                                            "/flex/flex.json",
                                        ]
                                        .join(""),
                                    )
                                    .unwrap();
                                    let mut data = String::new();
                                    file.read_to_string(&mut data).unwrap();
                                    let json: Value = serde_json::from_str(&data).unwrap();
                                    if wants.contains("..") {
                                        "404.html".to_string()
                                    } else if json[wants[1..].to_string()] != Value::Null {
                                        "video.html".to_string()
                                    } else if wants.starts_with("/assets/") {
                                        format!(".{}", wants)
                                    } else if wants.starts_with("/videos/")
                                        && json[wants[8..].to_string()] != Value::Null
                                    {
                                        format!(
                                            "{}",
                                            json[wants[8..].to_string()]
                                                .to_string()
                                                .trim_matches('\"')
                                                .to_string()
                                        )
                                    } else {
                                        "404.html".to_string()
                                    }
                                } else {
                                    "404.html".to_string()
                                }
                            }
                        };
                        println!("{} : {}", response, file_wants);
                        let mut f = File::open(file_wants.clone()).expect("no file found");
                        let mut buffer = Vec::new();
                        let file_length: usize = fs::metadata(file_wants.clone())
                            .unwrap()
                            .len()
                            .try_into()
                            .unwrap();
                        if range != (0, 0) && range.1 == 0 {
                            range = (range.0, file_length);
                        }
                        if file_wants.ends_with(".css") {
                            for i in
                                "HTTP/1.1 200 Ok\r\nContent-type: text/css; charset=utf-8\r\n\r\n"
                                    .as_bytes()
                            {
                                buffer.push(*i);
                            }
                        } else if file_wants.ends_with(".js") {
                            for i in "HTTP/1.1 200 Ok\r\nContent-type: text/javascript; charset=utf-8\r\n\r\n".as_bytes() {
                                buffer.push(*i);
                            }
                        } else if wants.starts_with("/videos/") && file_wants != "404.html" {
                            let length = if range != (0, 0) {
                                range.1 - range.0
                            } else {
                                file_length
                            };
                            let code = if range != (0, 0) { 206 } else { 200 };
                            let content_rang = if range != (0, 0) {
                                format!(
                                    "\r\nContent-Range: bytes {}-{}/{}",
                                    range.0, range.1, file_length
                                )
                            } else {
                                "".to_string()
                            };
                            for i in format!(
                                "HTTP/1.1 {} Ok{}\r\nContent-Type: {}\r\nAccept-Ranges: bytes\r\nContent-Length: {}\r\n\r\n",
                                code,
                                content_rang,
                                infer::get_from_path(file_wants.clone())
                                    .expect("file read successfully")
                                    .expect("file type is known")
                                    .mime_type(),
                                    length
                            )
                            .as_bytes()
                            {
                                buffer.push(*i);
                            }
                        } else {
                            for i in "HTTP/1.1 200 Ok\r\n\r\n".as_bytes() {
                                buffer.push(*i);
                            }
                        }
                        if wants.starts_with("/videos/")
                            && file_wants != "404.html"
                            && range != (0, 0)
                        {
                            let mut buffer_video = Vec::new();
                            f.read_to_end(&mut buffer_video).expect("buffer overflow");
                            buffer = [buffer, buffer_video[range.0..range.1].to_vec()].concat();
                        } else {
                            f.read_to_end(&mut buffer).expect("buffer overflow");
                        }
                        stream.write(&buffer).unwrap();
                        stream.flush().unwrap();
                    }
                });
            }
        }
    }
}

fn write_file(file: String, text: String) -> io::Result<()> {
    let mut file = File::create(file)?;
    file.write_all(text.trim().as_ref())?;
    Ok(())
}

fn make_error(beginning_text: &str, why: io::Error) {
    println!("{}{:?}", beginning_text, why);
    process::exit(1);
}

fn input(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().expect("flush failed!");
    let mut ret = String::new();
    io::stdin()
        .read_line(&mut ret)
        .expect("Failed to read from stdin");
    ret
}

fn add_item_to_json(movie_id: String, location_on_computer: String, location_of_json: String) {
    let file_cont = fs::read_to_string(location_of_json.clone()).expect("Unable to read file");
    let mut write: String = file_cont.chars().collect::<Vec<char>>()[..file_cont.len() - 2]
        .into_iter()
        .collect();
    if file_cont.chars().nth(file_cont.len() - 3).unwrap() == '\"' {
        write.push(',');
    }
    write.push_str(format!("\n\t\"{}\":\"{}\"\n", movie_id, location_on_computer).as_str());
    write.push('}');
    if let Err(why) = write_file(location_of_json, write) {
        make_error("Error writing to flex config: ", why);
    }
}
