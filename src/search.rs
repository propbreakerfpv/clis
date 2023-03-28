use std::{fs, path::PathBuf};
use ignore::Walk;


#[derive(Debug, Clone)]
pub struct SearchResult {
    pub path: PathBuf,
    pub context: String,
}

pub struct Filter {
    pub filters: String, // tmp type
    pub filter_type: FilterType,
}

pub enum FilterType {
    Gitignore,
    Normal,
}

pub fn search_dir(path: PathBuf, search_term: &String, filter: &Filter) -> Vec<SearchResult> {
    match &filter.filter_type {
        FilterType::Gitignore => {
            search_git_dir(path, search_term)
        }
        FilterType::Normal => {
            search_normal_dir(path, search_term)
        }
    }
}

fn search_git_dir(path: PathBuf, search_term: &String) -> Vec<SearchResult> {

    let mut search_results = Vec::new();

    for file in Walk::new(path) {
        let file = match file {
            Ok(f) => f,
            Err(_) => continue,
        };

        if file.file_type().unwrap().is_dir() {
            continue;
        }
        match search_file(file.path().to_path_buf(), &search_term) {
            Some(mut search) => {
                search_results.append(&mut search);
            }
            None => {}
        };

    }
    search_results
}

fn search_normal_dir(path: PathBuf, search_term: &String) -> Vec<SearchResult> {
    let mut search_results = Vec::new();
    let folder = fs::read_dir(&path);
    if folder.is_ok() {
        for file in folder.unwrap() {
            let file = file.unwrap();

            if file.file_type().unwrap().is_dir() {
                search_normal_dir(file.path(), search_term)
                    .iter()
                    .for_each(|x| search_results.push(x.clone()));
                continue;
            }
            match search_file(file.path(), &search_term) {
                Some(mut search) => {
                    search_results.append(&mut search);
                }
                None => {}
            };
        }
        return search_results;
    } else {
        return search_file(path.clone(), search_term).unwrap_or_else(|| panic!("{:?} not found", path));
    }
}

pub fn search_file(path: PathBuf, search_term: &String) -> Option<Vec<SearchResult>> {
    let content = fs::read_to_string(&path).unwrap_or(String::from(""));
    if content.find(search_term).is_none() {
        return None;
    }
    let lines: Vec<_> = content
        .lines()
        .collect();

    let mut prev: &str = "";
    let mut contexts = Vec::new();
    let mut after = false;
    let mut line_number = 1;
    for line in lines {
        if line.contains(search_term) {
            let mut context = String::from(format!("{} | {}\n", line_number - 1, prev));
            context.push_str(format!("{} | {}\n", line_number, line).as_str());
            let result = SearchResult {
                path: path.clone(),
                context,
            };
            contexts.push(result);
            after = true;
        } else if after {
            let idx = contexts.len() - 1;
            contexts.get_mut(idx).unwrap().context.push_str(format!("{} | {}", line_number, line).as_str());
            after = false;
        }
        prev = line;
        line_number += 1;
    }
    return Some(contexts);
}

