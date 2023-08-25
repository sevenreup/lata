use std::{fs, path::PathBuf};

use clap::Parser;
use handlebars::Handlebars;
use std::collections::BTreeMap;

#[derive(Debug)]
struct Site {
    base_path: String,
    index_html: String,
    markdown_files: Vec<MarkdownFile>,
}

#[derive(Debug)]
struct MarkdownFile {
    path: PathBuf,
    relative_path: String,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);

    let path = args.path;

    let html_string = read_index_html(&path);
    let markdown_files = read_markdown_files(&path);
    let site = Site {
        base_path: path,
        index_html: html_string,
        markdown_files,
    };
    generate_html_files(&site)
}

fn read_index_html(path: &String) -> String {
    let index_path = format!("{}/index.html", path);
    if !fs::metadata(&index_path).is_ok() {
        return "".to_string();
    } else {
        let index_html = fs::read_to_string(&index_path).unwrap();
        println!("{}", index_html);
        return index_html;
    }
}

fn read_markdown_files(base_path: &String) -> Vec<MarkdownFile> {
    let mut files: Vec<MarkdownFile> = Vec::new();
    let mut dirs: Vec<PathBuf> = Vec::new();
    let base_path = fs::canonicalize(base_path).unwrap();
    dirs.push(base_path.clone());

    while dirs.len() > 0 {
        let dir = dirs.pop().unwrap();
        let paths = fs::read_dir(dir).unwrap();
        for path in paths {
            let path = path.unwrap().path();
            if path.is_dir() {
                dirs.push(path);
            } else {
                if path.to_str().unwrap().ends_with(".md") {
                    let relative_path = path.strip_prefix(base_path.clone()).unwrap();
                    let relative_path = relative_path.parent().unwrap().to_str().unwrap();
                    files.push(MarkdownFile {
                        path: path.clone(),
                        relative_path: relative_path.to_string(),
                    });
                }
            }
        }
    }
    for file in &files {
        println!("{:?}", file);
    }

    return files;
}

fn generate_html_files(site: &Site) {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_string("t1", &site.index_html)
        .unwrap();

    let files = &site.markdown_files;

    create_and_clean_build_dir(&site.base_path);

    for file in files {
        let source = fs::read_to_string(&file.path).unwrap();
        let mut data = BTreeMap::new();
        data.insert("content".to_string(), source);
        let result = handlebars.render("t1", &data).unwrap();
        println!("{}", result);

        let build_path = format!("{}/build/{}", site.base_path, file.relative_path);
        let build_path = build_path.replace("\\", "/");
        let build_path = format!("{}/index.html", build_path);

        let mut file_folder = PathBuf::from(&build_path);
        file_folder.pop();

        if !fs::metadata(&file_folder).is_ok() {
            fs::create_dir_all(&file_folder).unwrap();
        }

        let file_path = PathBuf::from(&build_path);

        fs::write(file_path, result).unwrap();
    }
}

fn create_and_clean_build_dir(base_path: &String) {
    let build_path = format!("{}/build", base_path);
    if fs::metadata(&build_path).is_ok() {
        fs::remove_dir_all(&build_path).unwrap();
    }
    fs::create_dir(&build_path).unwrap();
}
