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
            Some(search) => {
                search_results.push(search);
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
                Some(search) => {
                    search_results.push(search);
                }
                None => {}
            };
        }
        return search_results;
    } else {
        return vec![search_file(path.clone(), search_term).unwrap_or_else(|| panic!("{:?} not found", path))]
    }
}

pub fn search_file(path: PathBuf, search_term: &String) -> Option<SearchResult> {
    let content = fs::read_to_string(&path).unwrap_or(String::from(""));
    let index = content.find(search_term);
    if index.is_some() {
        let line: Vec<_> = content
            .lines()
            .filter(|&x| x.contains(search_term))
            .map(|x| x.to_string())
            .collect();
        let start_index = line
            .get(0)
            .unwrap_or(&String::new())
            .find(search_term);

        let mut context = String::new();

        if start_index.is_some() {
            let start_index = start_index.unwrap();
            let mut end_index = line[0].len();
            if end_index > start_index + 50 {
                end_index = start_index + 50;
            }
            context = line[0][start_index..end_index].to_string();
        }

        return Some(SearchResult { path, context });
    }
    return None;
}
