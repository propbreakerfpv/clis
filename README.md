# clis (cli search)
clis is a simple find/grep/fuzzy finder like command used to find all matches 
to a search in a given folder or file.
if a path to a folder is provided clis will search all text files in that 
folder for the search term, if no path is provided clis will search in the
present working directory

## Install
`cargo install clis`

## Usage
Usage: clis <search term> [opt path]
flags:
  --version         displays the version of clis currently installed on the machine
  -i | --gitignore  when used this flag means that clis searches in directorys ignored by git
  -h | --help       displays this menue
