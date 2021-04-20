use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

//TODO: Add support for tags h2-h6, img, a, b, i, ul/li, table and emoji!

fn md_to_html(file: &str){
    println!("Parsing {} to html...", file);
    
    let input = Path::new(file);

    let md_file = File::open(&input).expect("No Such File!");

    let mut _ptag: bool = false;
    let mut _h1tag: bool = false;
    // let mut _h2tag: bool = false;
    // let mut _h3tag: bool = false;
    // let mut _h4tag: bool = false;
    // let mut _h5tag: bool = false;
    // let mut _h6tag: bool = false;
    // let mut _atag: bool = false;
    // let mut _btag: bool = false;
    // let mut _itag: bool = false;
    
    // the html output generated as a vector of strings;
    let mut tokens: Vec<String> = Vec::new();

    //push the basic starting html, head, and title tags tags;
    tokens.push(String::from("<html>\n<head>\n<title>"));
    tokens.push(String::from(file));
    tokens.push(String::from("</title>\n</head>\n<body>\n"));
    
    //the bufreader for reading the md file line by line
    let reader = BufReader::new(md_file);

    for line in reader.lines() {
        let contents = line.unwrap();
        
        let mut first: Vec<char> = contents.chars().take(1).collect();
        let mut output = String::new();

        match first.pop(){
            Some('#') => {
                if _ptag{
                    _ptag = false;
                    output.push_str("</p>\n");
                }
                if _h1tag{
                    _h1tag = false;
                    output.push_str("</h1>\n");
                }
                _h1tag = true;
                output.push_str("<h1>");
                output.push_str(&contents[2..]);
            },
            _ => {
                if !_ptag {
                    _ptag = true;
                    output.push_str("<p>");
                }

                output.push_str(&contents);

            }
        }

        //check for any open tags, if yes close them
        //and remove the empty tags before writing to tokens vector
        if _ptag{
            _ptag = false;
            output.push_str("</p>\n");
        }
        if _h1tag{
            _h1tag = false;
            output.push_str("</h1>\n");
        }
        if output != "<p></p>\n"{
            tokens.push(output);
        }
    }

    //push the closing body and html tags;
    tokens.push(String::from("</body>\n"));
    tokens.push(String::from("</html>"));

    /*for st in &tokens{
        println!("{}", st);
    }*/

    //create the file name wiht html extension;
    let mut output_fname = String::from(&file[..file.len()-3]);
    output_fname.push_str(".html");

    //create the actual file using the filename defined above;
    let mut outfile = File::create(output_fname).expect("Error creating new file!");

    //write each string in tokens to the html file;
    for line in &tokens {
        outfile.write_all(line.as_bytes()).expect("Could not write value!");
    }

    println!("Html generated from Markdown!");
}

fn title() -> String{
    let mut title = String::from(env!("CARGO_PKG_NAME"));
    title.push_str(" v");
    title.push_str(env!("CARGO_PKG_VERSION"));
    title.push_str(", ");
    title.push_str(env!("CARGO_PKG_DESCRIPTION"));

    title
}

fn intro(title:&mut String){
    println!("{}", title);
    println!("©binarypheonix");
}
fn details(){
    println!("Usage: mage <markdown-file>");
    println!("For more info, goto {}", env!("CARGO_PKG_HOMEPAGE"));
}

fn usage(){
    intro(&mut title());
    details();    
}

fn main() {
    let args:Vec<String> = env::args().collect();

    match args.len(){
        1 => {
            usage();
        },
        2 => {
            //read the contents of the markdown file
            //convert to html-ish syntax and write to html file
            md_to_html(&args[1]);
        },
        _ => {
            println!("More than one argument passed.");
            details();
        }
    }
}
