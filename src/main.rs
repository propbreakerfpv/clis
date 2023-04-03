use std::{path::PathBuf, env, collections::HashMap };

use search::Filter;

use crate::search::Filt;

mod search;
fn main() {

    let params = get_args(vec![
             Param {
                 flag: "--extention".to_string(),
                 short_hand: "-e".to_string(),
                 param_type: ParamType::MultipleArgs((None, Vec::new()))
             },
             Param {
                 flag: "--version".to_string(),
                 short_hand: "-v".to_string(),
                 param_type: ParamType::Flag,
             },
             Param {
                 flag: "--gitignore".to_string(),
                 short_hand: "-i".to_string(),
                 param_type: ParamType::Flag,
             },
             Param {
                 flag: "--help".to_string(),
                 short_hand: "-h".to_string(),
                 param_type: ParamType::Flag,
             }
    ]);

    // return;


    if args_has(&params, "-v") {
        println!("{}", env!("CARGO_PKG_VERSION"));
        std::process::exit(0);
    }
    if args_has(&params, "-h") {
        display_help();
        std::process::exit(0);
    }
    let mut args = env::args();
    args.next();
    let search_term = args.next().unwrap_or_else(|| {
        display_help();
        std::process::exit(0);
    });

    // let mut path = args.next().unwrap_or(env::current_dir().unwrap().to_str().unwrap().to_string());
    // if path.starts_with("--") {
    let path = env::current_dir().unwrap().to_str().unwrap().to_string();
    // }

    let mut filter = Filter {
        filters: Vec::new(),
        filter_type: search::FilterType::Gitignore
    };

    if args_has(&params, "-i") {
        println!("not ignored");
        filter.filter_type = search::FilterType::Normal;
    }

    let ext = params.get("-e");
    if let Some(param) = &ext {
        if let ParamType::MultipleArgs(v) = &param.param_type {
            v.1.iter().for_each(|x|{
                filter.filters.push(Filt::Extention(x.clone()))
            });
        }
    }

    let results = search::search_dir(PathBuf::from(path), &search_term, filter);


    for result in results {
        println!("{:?}", result.path);
        println!("{}", result.context);
    }
}


fn display_help() {
    println!("Usage: clis <search term>");
    println!("flags:");
    println!("  --version         displays the version of clis currently installed on the machine");
    println!("  -i | --gitignore  when used this flag means that clis searches in directorys ignored by git");
    println!("  -h | --help       displays this menue");
    println!("  -e | --extention [list of file extention] specify a list of file extentions to ignore");
}


#[derive(Debug, Clone)]
struct Param {
    flag: String,
    short_hand: String,
    param_type: ParamType,
}

impl Param {
    fn new(flag: String, short_hand: String, param_type: ParamType) -> Param {
        Param {
            flag,
            short_hand,
            param_type,
        }
    }
}

#[derive(Debug, Clone)]
enum ParamType {
    Flag,
    SingleArg(String),
    MultipleArgs((Option<u32>, Vec<String>)),
}

fn get_args(params: Vec<Param>) -> HashMap<String, Param> {
    let mut out = HashMap::new();
    let args_len = env::args().len();
    let mut params = params.into_iter();
    let mut param;
    let mut in_param = 0;
    let mut param_end_next = true;

    loop {
        param = match params.next() {
            Some(v) => v,
            None => break,
        };

        let mut idx = 1;
        for arg in env::args() {
            if in_param > 0 {
                in_param -= 1;
                if arg.starts_with("-") {
                    if param_end_next {
                        out.insert(param.short_hand.clone(), param.clone());
                        in_param = 0;
                    }
                } else {
                    match &param.param_type {
                        ParamType::Flag => {
                            println!("some internal errror ocered whitch should never happen!!!");
                        }
                        ParamType::SingleArg(_) => {
                            param.param_type = ParamType::SingleArg(arg.clone());
                        }
                        ParamType::MultipleArgs(v) => {
                            let mut vec = v.1.clone();
                            vec.push(arg.clone());
                            param.param_type = ParamType::MultipleArgs((v.0, vec));
                        }
                    }
                    if in_param == 0 || idx == args_len {
                        out.insert(param.short_hand.clone(), param.clone());
                        in_param = 0;
                    }
                }
            } else {
                if arg == param.short_hand.clone() || arg == param.flag {
                    match &param.param_type {
                        ParamType::Flag => {
                            out.insert(param.short_hand.clone(), param.clone());
                        }
                        ParamType::SingleArg(_) => {
                            in_param = 1;
                        }
                        ParamType::MultipleArgs(v) => {
                            match v.0 {
                                Some(i) => {
                                    in_param = i;
                                    param_end_next = false;
                                }
                                None => {
                                    in_param = args_len as u32;
                                }
                            }
                        }
                    }
                }
            }
            idx += 1;
        }
    }
    out
}

fn args_has<T: ToString>(params: &HashMap<String, Param>, arg: T) -> bool {
    params.get(&arg.to_string()).is_some()
}


