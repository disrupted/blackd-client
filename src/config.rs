use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use toml;

/// Represents the configuration loaded from the `pyproject.toml` file.
///
/// The struct is created using the `from_file()` or `load()` functions. It has a `tool` field
/// that contains an optional `ToolConfig` instance.
#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "tool")]
    pub tool: Option<ToolConfig>,
}

/// Represents the `tool` configuration section in the `pyproject.toml` file.
///
/// The struct is used in the `Config` struct to deserialize the `tool` section. It has an
/// optional `black` field that contains a `BlackConfig` instance.
#[derive(Debug, Deserialize)]
pub struct ToolConfig {
    pub black: Option<BlackConfig>,
}

/// Represents the `black` configuration section in the `pyproject.toml` file.
///
/// The struct is used in the `ToolConfig` struct to deserialize the `black` section. It has
/// an optional `line_length` field that specifies the maximum line length, and an optional
/// `target_version` field that specifies the target Python versions.
#[derive(Debug, Deserialize)]
pub struct BlackConfig {
    #[serde(rename = "line-length")]
    pub line_length: Option<i32>,
    #[serde(rename = "target-version")]
    pub target_version: Option<Vec<String>>,
}

impl Config {
    /// Loads the configuration from the `pyproject.toml` file.
    ///
    /// The function searches for the `pyproject.toml` file in the current directory and its
    /// parent directories. It returns a `Config` instance or a default instance if the file
    /// is not found.
    ///
    /// # Arguments
    ///
    /// * `target_file_name`: An optional `&str` that specifies the target file name. The
    /// default value is `"pyproject.toml"`.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = Config::load(None);
    /// println!("{:?}", config);
    /// ```
    pub fn load(target_file_name: Option<&str>) -> Self {
        let mut path = env::current_dir().unwrap();
        let target_file_name = target_file_name.unwrap_or("pyproject.toml");

        while path.pop() {
            let potential_config_path = path.join(target_file_name);
            if potential_config_path.is_file() {
                return Config::from_file(potential_config_path)
                    .expect("Failed to load config file");
            }
        }

        Config { tool: None }
    }

    /// Creates a `Config` instance by deserializing the content of the specified file.
    ///
    /// The function returns a `Config` instance or an error if deserialization fails.
    ///
    /// # Arguments
    ///
    /// * `path`: A `PathBuf` instance that specifies the path to the file.
    ///
    /// # Examples
    ///
    /// ```
    /// let path = std::path::PathBuf::from("pyproject.toml");
    /// let config = Config::from_file(path);
    /// println!("{:?}", config);
    /// ```
    pub fn from_file(path: impl Into<PathBuf>) -> Result<Self, std::io::Error> {
        let path = path.into();
        let mut file = File::open(&path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config = toml::from_str(&contents).unwrap();
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;

    fn create_test_config_file(content: &str, file_name: &str) -> std::io::Result<PathBuf> {
        let mut file_path = env::temp_dir();
        file_path.push(file_name);
        let mut file = File::create(&file_path)?;
        file.write_all(content.as_bytes())?;
        Ok(file_path)
    }

    fn delete_test_config_file(file_path: &PathBuf) {
        let _ = std::fs::remove_file(&file_path);
    }

    #[test]
    fn test_from_file() {
        let config_content = r#"
        [tool]
        [tool.black]
        line-length = 80
        target-version = ['py36', 'py37']
    "#;

        let file_path = create_test_config_file(config_content, "test_from_file.toml").unwrap();

        let config = Config::from_file(&file_path).unwrap();
        let black_config = config.tool.unwrap().black.unwrap();
        assert_eq!(black_config.line_length.unwrap(), 80);
        assert_eq!(black_config.target_version.unwrap(), vec!["py36", "py37"]);

        delete_test_config_file(&file_path);
    }

    #[test]
    fn test_load() {
        let config_content = r#"
        [tool]
        [tool.black]
        line-length = 100
        target-version = ['py38', 'py39']
    "#;

        let file_path = create_test_config_file(config_content, "test_load.toml").unwrap();
        let prev_current_dir = env::current_dir().unwrap();

        let mut parent_dir = file_path.clone();
        parent_dir.pop();
        let _ = env::set_current_dir(&parent_dir);

        let config = Config::load(Some(&file_path.to_str().unwrap()));
        let black_config = config.tool.unwrap().black.unwrap();
        assert_eq!(black_config.line_length.unwrap(), 100);
        assert_eq!(black_config.target_version.unwrap(), vec!["py38", "py39"]);

        delete_test_config_file(&file_path);
        let _ = env::set_current_dir(prev_current_dir);
    }

    #[test]
    fn test_empty_config_from_file() {
        let config_content = r#""#;
        let file_path = create_test_config_file(config_content, "test_empty.toml").unwrap();

        let config = Config::from_file(&file_path).unwrap();
        assert!(config.tool.is_none());

        delete_test_config_file(&file_path);
    }

    #[test]
    fn test_partial_config_from_file() {
        let config_content = r#"
            [tool.black]
            line-length = 120
        "#;

        let file_path = create_test_config_file(config_content, "test_partial.toml").unwrap();
        let config = Config::from_file(&file_path).unwrap();

        assert!(config.tool.is_some());
        let tool = config.tool.unwrap();

        assert!(tool.black.is_some());

        let black = tool.black.unwrap();
        assert_eq!(black.line_length, Some(120));
        assert!(black.target_version.is_none());

        delete_test_config_file(&file_path);
    }
}
