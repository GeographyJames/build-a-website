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
    fn from_file_path(file_path: PathBuf) -> Result<Post, PostError> {
        let file_name;

        if let Some(f) = file_path.file_name() {
            file_name = f
        } else {
            return Err(PostError::new("file has no name"));
        }

        let file_name_string;
        if let Some(str) = file_name.to_str() {
            file_name_string = str;
        } else {
            return Err(PostError::new("could not convert file name to string"));
        }
        let mut file_name_split = file_name_string.split('.');

        let post_date_and_name;
        if let Some(str) = file_name_split.next() {
            post_date_and_name = str
        } else {
            return Err(PostError::new("unable to create post name"));
        };

        let file_extension;
        if let Some(str) = file_name_split.next() {
            file_extension = str
        } else {
            return Err(PostError::new("no file extension"));
        }

        if file_extension != "mk" {
            return Err(PostError::new("invalid file extension"));
        }

        let mut file_name_iterator = post_date_and_name.split('-');
        let year;
        if let Some(str) = file_name_iterator.next() {
            if let Ok(y) = str.parse::<i32>() {
                year = y
            } else {
                return Err(PostError::new("could not parse year"));
            }
        } else {
            return Err(PostError::new("no year component of name"));
        }

        let month;
        if let Some(str) = file_name_iterator.next() {
            if let Ok(m) = str.parse::<u32>() {
                month = m
            } else {
                return Err(PostError::new("could not parse month"));
            }
        } else {
            return Err(PostError::new("no month component of name"));
        }
        let day;
        if let Some(str) = file_name_iterator.next() {
            if let Ok(d) = str.parse::<u32>() {
                day = d
            } else {
                return Err(PostError::new("could not parse day"));
            }
        } else {
            return Err(PostError::new("no day component of name"));
        };
        let date;
        if let Some(res) = NaiveDate::from_ymd_opt(year, month, day) {
            date = res
        } else {
            return Err(PostError::new("could not parse date"));
        }
        let title = file_name_iterator.collect::<Vec<&str>>().join("-");
        if title.is_empty() {
            return Err(PostError::new("post has no title"));
        }
        Ok(Post { title, date })
    }
}

#[derive(Debug)]
struct PostError {
    message: String,
}

impl fmt::Display for PostError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl PostError {
    fn new(message: &str) -> PostError {
        PostError {
            message: message.to_string(),
        }
    }
}

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
            Post::from_file_path(PathBuf::from_str("my_directory/2000-01-01-my-post.mk").unwrap())
                .unwrap(),
            output
        );
    }

    #[test]
    fn should_fail_because_file_has_no_name() {
        let result = Post::from_file_path(PathBuf::from_str("").unwrap());
        assert!(result.is_err());
    }

    #[test]
    fn should_fail_because_file_has_no_name_and_provide_message() {
        let result = Post::from_file_path(PathBuf::from_str("").unwrap());
        if let Err(err) = result {
            assert!(err.message == "file has no name")
        } else {
            panic!()
        };
    }

    #[test]
    fn should_fail_because_file_has_no_extension() {
        let result =
            Post::from_file_path(PathBuf::from_str("my_directory/2000-01-01-my-post").unwrap());
        if let Err(err) = result {
            assert!(err.message == "no file extension")
        } else {
            panic!()
        }
    }

    #[test]
    fn should_fail_because_file_extension_is_not_mk() {
        let result = Post::from_file_path(
            (PathBuf::from_str("my_directory/2000-01-01-my-post.ext")).unwrap(),
        );
        if let Err(err) = result {
            assert!(err.message == "invalid file extension")
        } else {
            panic!()
        }
    }

    #[test]
    fn should_fail_because_name_has_no_year() {
        let result =
            Post::from_file_path((PathBuf::from_str("my_directory/-01-01-my-post.mk")).unwrap());
        if let Err(err) = result {
            assert!(err.message == "could not parse year")
        } else {
            panic!()
        }
    }
    #[test]
    fn should_fail_because_name_has_no_month() {
        let result = Post::from_file_path(
            (PathBuf::from_str("my_directory/2000-month-01-my-post.mk")).unwrap(),
        );
        if let Err(err) = result {
            println!("{}", err.message);
            assert!(err.message == "could not parse month")
        } else {
            panic!()
        }
    }

    #[test]
    fn should_fail_because_name_has_no_day() {
        let result = Post::from_file_path(
            (PathBuf::from_str("my_directory/2000-01-day-my-post.mk")).unwrap(),
        );
        if let Err(err) = result {
            println!("{}", err.message);
            assert!(err.message == "could not parse day")
        } else {
            panic!()
        }
    }
    #[test]
    fn should_fail_because_date_invalid() {
        let result = Post::from_file_path(
            (PathBuf::from_str("my_directory/2000-111-01-my-post.mk")).unwrap(),
        );
        if let Err(err) = result {
            println!("{}", err.message);
            assert!(err.message == "could not parse date")
        } else {
            panic!()
        }
    }
    #[test]
    fn should_fail_because_name_has_no_title() {
        let result =
            Post::from_file_path((PathBuf::from_str("my_directory/2000-01-01.mk")).unwrap());
        if let Err(err) = result {
            println!("{}", err.message);
            assert!(err.message == "post has no title")
        } else {
            panic!()
        }
    }
}
