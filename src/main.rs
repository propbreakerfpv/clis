use std::{fs, path::PathBuf};

fn main() {
    let results = search_dir(PathBuf::from("src/main.rs"), &"fuck".to_string());
    for result in results {
        println!("{:?} -> {}", result.path, result.context);
    }
}

#[derive(Debug, Clone)]
struct SearchResult {
    path: PathBuf,
    context: String,
}

fn search_dir(path: PathBuf, search_term: &String) -> Vec<SearchResult> {
    let mut search_results = Vec::new();
    let folder = fs::read_dir(&path);
    if folder.is_ok() {
        for file in folder.unwrap() {
            let file = file.unwrap();
            if file.file_type().unwrap().is_dir() {
                search_dir(file.path(), search_term)
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
        return vec![match search_file(path, search_term) {
            Some(search) => search,
            None => panic!(),
        }];
    }
}

fn search_file(path: PathBuf, search_term: &String) -> Option<SearchResult> {
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
