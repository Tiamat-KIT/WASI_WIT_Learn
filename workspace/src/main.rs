// https://toranoana-lab.hatenablog.com/entry/2023/03/02/100000
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();

    if args.len() < 2 {
        eprintln!("usage: {} <filepath>", program);
        return;
    }

    let contents = match read_markdown(&args[1]) {
        Ok(c) => c,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    let html = parse_markdown(&contents);
    match write_html(&html) {
        Ok(_) => println!("success markdown to html. look at index.html"),
        Err(err) => {
            eprintln!("{}", err);
        }
    };
}

fn read_markdown(file_name: &str) -> Result<String,String>{
    Ok(std::fs::read_to_string(file_name).map_err(|err| format!("error opening input {}: {}", file_name, err))?)
}

fn parse_markdown(markdown: &str) -> String {
    use pulldown_cmark::{Options,Parser,html};
    use ammonia::clean;

    let mut opt = Options::empty();
    opt.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(markdown, opt);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    clean(&*html_output)
}

fn write_html(html: &str) -> Result<(),String> {
    use std::fs::File;
    use std::path::Path;
    use std::io::Write;

    let path = Path::new("./index.html");

    let mut file = File::create(&path).map_err(
        |err| format!("error write file index.html {}",err)
    )?;

    file.write_all(html.as_bytes()).map_err(|err| format!("Error write file index.html: {}",err))?;
    Ok(())
}

