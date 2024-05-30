use chrono::NaiveDate;
use core::fmt;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::PathBuf;
use yaml_front_matter::YamlFrontMatter;

#[derive(PartialEq, Debug, Clone, Serialize)]
pub struct Post {
    pub frontmatter: PostFrontmatter,
    pub source_file_path: PathBuf,
    pub file_name: String,
    pub title: String,
    pub date: NaiveDate,
    pub collection: String,
    pub url: String,
}

#[derive(PartialEq, Debug, Clone, Deserialize, Serialize)]
pub struct PostFrontmatter {
    topic: String,
    description: String,
    title: String,
}

impl Post {
    pub fn from_file_path(file_path: &PathBuf) -> Result<Post, PostError> {
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

        if file_extension != "md" {
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
        let title = file_name_iterator.collect::<Vec<&str>>().join(" ");
        if title.is_empty() {
            return Err(PostError::new("post has no title"));
        }

        let content;
        if let Ok(str) = std::fs::read_to_string(file_path) {
            content = str
        } else {
            return Err(PostError::new("could not read markdown file"));
        }

        let yaml_result;
        if let Ok(res) = YamlFrontMatter::parse::<PostFrontmatter>(&content) {
            yaml_result = res;
        } else {
            return Err(PostError::new("failed to create YAML frontmatter"));
        }

        let file_name = post_date_and_name.to_string();

        let parent;
        if let Some(path) = file_path.parent() {
            parent = path;
        } else {
            return Err(PostError::new("path has no parent"));
        }

        let components = parent
            .components()
            .skip(1)
            .map(|c| c.as_os_str().to_str().unwrap().to_string())
            .collect::<Vec<String>>();
        let collection;
        if components.len() > 1 {
            return Err(PostError::new("directories nested too deep."));
        }
        if let Some(str) = components.last() {
            collection = str.clone()
        } else {
            return Err(PostError::new("post does not belong to a collection"));
        }
        let url = format!("{}/{}", collection, file_name);

        Ok(Post {
            frontmatter: yaml_result.metadata,
            source_file_path: file_path.clone(),
            file_name,
            title,
            date,
            collection,
            url,
        })
    }
}

#[derive(Debug, Clone)]
pub struct PostError {
    pub message: String,
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
