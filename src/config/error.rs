use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

#[derive(Debug)]
pub enum ConfigError {
    // 找不到配置文件
    NotFound,
    // 读取错误
    IoError,
    // 文件路径无效
    BadFilePath(PathBuf, &'static str),
    // 指定的环境无效
    BadEnv(String),
    // 读取文件时，文件内容有问题
    BadEntry(String, PathBuf),
    // 类型错误
    BadType(String, &'static str, &'static str, Option<PathBuf>),
    // 语法解析错误
    ParseError(String, PathBuf, String, Option<(usize, usize)>),
}

impl Error for ConfigError {
    fn description(&self) -> &str {
        use self::ConfigError::*;
        match *self {
            NotFound => "config file was not found",
            IoError => "there was an I/O error while reading the config file",
            BadFilePath(..) => "the config file path is invalid",
            BadEnv(..) => "the environment specified in `ROCKET_ENV` is invalid",
            BadEntry(..) => "an environment specified as `[environment]` is invalid",
            BadType(..) => "a key was specified with a value of the wrong type",
            ParseError(..) => "the config file contains invalid TOML",
        }
    }
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use self::ConfigError::*;
        match *self {
            NotFound => write!(f, "config file was not found"),
            IoError => write!(f, "I/O error while reading the config file"),
            BadFilePath(ref p, _) => write!(f, "{:?} is not a valid config path", p),
            BadEnv(ref e) => write!(f, "{:?} is not a valid `ROCKET_ENV` value", e),
            BadEntry(ref e, _) => {
                write!(f, "{:?} is not a valid `[environment]` entry", e)
            }
            BadType(ref n, e, a, _) => {
                write!(f, "type mismatch for '{}'. expected {}, found {}", n, e, a)
            }
            ParseError(..) => write!(f, "the config file contains invalid TOML"),
        }
    }
}
