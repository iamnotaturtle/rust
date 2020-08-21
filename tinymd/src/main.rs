fn get_title() -> String {
    return format!("{} (v{}), {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_DESCRIPTION"));
}

fn parse_markdown_file(_filename: &str) {
    print_short_banner();
    println!("[ INFO ] Trying to parse {}...", _filename)
}

fn print_short_banner() {
    println!("{}", get_title());
}

fn print_long_banner() {
    print_short_banner();
    println!("Written by: {}\nHomepage: {}\nUsage: tinymd <somefile>.md\n", 
        env!("CARGO_PKG_AUTHORS"), 
        env!("CARGO_PKG_HOMEPAGE")
    );
}

fn usage() {
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        2 => parse_markdown_file(&args[1]),
        _ => {
            println!("[ Error ] Invalid innvocation (you done goofed!)");
            usage();
        },
    }
}
