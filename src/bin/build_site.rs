use core::fmt;
use std::io::Write;
use std::num::ParseIntError;
use std::{error::Error, fs, path::PathBuf};

use chrono::NaiveDate;
use comrak::{markdown_to_html, Options};

fn main() -> Result<(), std::io::Error> {
    create_directories();
    parse_markdown_posts();
    let test_post = Post {
        title: "a test post".to_string(),
        date: NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
    };
    Ok(())
}

#[derive(PartialEq, Debug)]
pub struct Post {
    title: String,
    date: NaiveDate,
}

impl Post {
    fn from_file_path(file_path: PathBuf) -> Post {
        let path_string = file_path.to_str().unwrap().to_string();
        let mut path_string_iterator = path_string.split('-');
        let year = path_string_iterator.next().unwrap().parse::<i32>().unwrap();
        let month = path_string_iterator.next().unwrap().parse::<u32>().unwrap();
        let day = path_string_iterator.next().unwrap().parse::<u32>().unwrap();
        let title = path_string_iterator.collect::<Vec<&str>>().join("-");
        Post {
            title,
            date: NaiveDate::from_ymd_opt(year, month, day).unwrap(),
        }
    }
}

impl fmt::Display for PostError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "something went wrong")
    }
}
#[derive(Debug)]
struct PostError;

impl Error for PostError {}

fn parse_markdown_posts() {
    for result in fs::read_dir("posts").expect("Could not access posts directory") {
        let directory_entry = result.unwrap();
        if directory_entry
            .file_type()
            .expect("could not determine file type")
            .is_file()
        {
            let file_path = directory_entry.path();
            if file_path.extension().is_some_and(|ext| ext == "mk") {
                let file_name = file_path
                    .file_stem()
                    .expect("file has no name")
                    .to_str()
                    .unwrap();
                let input = fs::read_to_string(&file_path).expect("could not read file to string");
                let output = markdown_to_html(&input, &Options::default());
                let mut f =
                    fs::File::create_new(format!("_site/posts/{}.html", file_name)).unwrap();
                let _ = write!(f, "{}", output);
            }
        }
    }
}

fn create_directories() {
    let _ = fs::remove_dir_all("_site");
    fs::create_dir_all("_site/posts").expect("Failed to create site directories.");
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    #[test]
    fn should_return_markdown() {
        let input = "Hello World";
        let output = "<p>Hello World</p>\n";
        assert_eq!(
            comrak::markdown_to_html(input, &comrak::Options::default()),
            output
        );
    }
    #[test]
    fn should_parse_markdown_file_name_and_return_post_struct() {
        let output = Post {
            title: "my-post".to_string(),
            date: NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
        };
        assert_eq!(
            Post::from_file_path(PathBuf::from_str("2000-01-01-my-post").unwrap()),
            output
        );
    }
}
