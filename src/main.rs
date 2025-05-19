use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("");
   

    match command {
        "-h" | "-help" => help_screen(),
        "-i" | "-install" => installation(),
        "-g" | "-get" => get_dots(),
        "-p" | "-push" => push_dots(),
        "-v" | "-version" => print_version(),
        "" => {
            println!("No command entered, see '-h' for options");
            std::process::exit(1);
        }
        _ => {
            println!("Unknown option: {}", command);
            help_screen();
        }
    }
    
}

fn help_screen() {
    println!(
        "
usage: dvc [options]
  Options:
  -help    || -h  - Prints this screen
  -install || -i  - Installs dotfiles into system
  -get     || -g  - Gets dotfiles
  -push    || -p  - Pushes dotfiles
  -version || -v  - Version of tool
        "
    );
}

fn print_version(){
    println!("Current version: v1.2.0-beta");
    // extra info
    println!("Extra info:");
    println!("Implemented first rust file");
}

fn installation(){
    println!("Installing dotfiles...");
    // TODO: install logic
}

fn get_dots(){
    println!("Cloning dotfiles...");
    // TODO: git clone logic
}

fn push_dots(){
    println!("Pushing dotfiles...");
    // TODO: git push
}
