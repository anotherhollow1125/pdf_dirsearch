# How to use

```
$ ./pdf_dirsearch --help
Search for a string in all PDF files in a directory

Usage: pdf_dirsearch [OPTIONS] --regex <REGEX>

Options:
  -r, --regex <REGEX>  
  -p, --path <PATH>    [default: .]
  -h, --help           Print help
  -V, --version        Print version
```

Please pass target directory path and search word as regex, and this application will full text search over files in the target directory.

⚠ This application creates a file converted from "xx.pdf" to "xx.txt". If there is already a "xx.txt" file in the directory, this may lead to undesired results.