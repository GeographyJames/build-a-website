use crate::post::{Post, PostError};
use std::io::Write;
use std::str::FromStr;
use std::{fs, path::PathBuf};

use comrak::{markdown_to_html, Options};

pub fn build() -> Vec<Result<Post, PostError>> {
    let source_directory = PathBuf::from_str("posts").unwrap();
    let target_directory = PathBuf::from_str("_site/posts").unwrap();
    fs::remove_dir_all(&target_directory).expect("should be able to remove");
    fs::create_dir_all(&target_directory).expect("should be able to create");
    let mut posts: Vec<Result<Post, PostError>> = Vec::new();
    let directory_entry = fs::read_dir(source_directory).unwrap();
    for directory in directory_entry {
        let post_result = Post::from_file_path(&(directory.unwrap().path()));
        posts.push(post_result.clone());
        if let Ok(post) = post_result {
            let input = fs::read_to_string(&post.source_file_path).unwrap();
            let output = markdown_to_html(&input, &Options::default());
            let mut target_filename = target_directory.clone();
            target_filename.push(format!("{}.html", &post.file_name));
            let mut html_file = fs::File::create_new(target_filename).unwrap();
            let _ = write!(html_file, "{}", output);
        }
    }
    posts
}
