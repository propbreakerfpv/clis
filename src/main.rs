use std::{path::PathBuf, env};

use search::Filter;

mod search;
fn main() {
    let mut args = env::args();

    if args_has("--version") {
        println!("{}", env!("CARGO_PKG_VERSION"));
        std::process::exit(0);
    }
    if args_has("--help") || args_has("-h") {
        display_help();
        std::process::exit(0);
    }
    args.next();
    let search_term = args.next().unwrap_or_else(|| {
        display_help();
        std::process::exit(0);
    });

    let mut path = args.next().unwrap_or(env::current_dir().unwrap().to_str().unwrap().to_string());
    if path.starts_with("--") {
        path = env::current_dir().unwrap().to_str().unwrap().to_string();
    }

    let mut filter = Filter {
        filters: String::new(),
        filter_type: search::FilterType::Gitignore
    };

    if args_has("--gitignore") || args_has("-i") {
        filter.filter_type = search::FilterType::Normal;
    }

    let results = search::search_dir(PathBuf::from(path), &search_term, &filter);
    for result in results {
        println!("{:?}", result.path);
        println!("{}", result.context);
    }
}


fn display_help() {
    println!("Usage: clis <search term> [opt path]");
    println!("flags:");
    println!("  --version         displays the version of clis currently installed on the machine");
    println!("  -i | --gitignore  when used this flag means that clis searches in directorys ignored by git");
    println!("  -h | --help       displays this menue");
}


fn args_has<T: ToString>(arg: T) -> bool {
    for i in env::args(){
        if i.contains(arg.to_string().as_str()) {
            return true;
        }
    }
    return false;
}

