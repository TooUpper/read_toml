use std::io::Read;

use crate::environment::Environment;

use super::{basic_config::BasicConfig, error::ConfigError};
use crate::env;
use crate::environment::Environment::{Development, Production, Staging};
use crate::fs;
use crate::File;
use crate::HashMap;
use crate::Path;
use crate::PathBuf;

const CONFIG_FILENAME: &str = "config/env_config.toml";

// 在 doc 中隐藏某个项
#[doc(hidden)]
#[derive(Debug, PartialEq)]
pub struct EnvConfig {
    // 当前是何种环境 development、staging、production
    pub active_env: Environment,
    // 环境对应的配置项
    config: HashMap<Environment, BasicConfig>,
}

impl EnvConfig {
    // find() 查找
    fn find() -> Result<PathBuf, ConfigError> {
        // 以 PathBuf 形式返回当前的目录
        // .map_err 用于将 Result 中的错误（Err变体）映射（转换）为另一种错误类型。
        let cwd: PathBuf = env::current_dir().map_err(|_| ConfigError::NotFound)?;
        // 转换为 &path
        let mut current: &Path = cwd.as_path();

        loop {
            let manifest = current.join(CONFIG_FILENAME);
            if fs::metadata(&manifest).is_ok() {
                return Ok(manifest);
            }

            match current.parent() {
                Some(p) => current = p,
                None => break,
            }
        }

        Err(ConfigError::NotFound)
    }

    fn git_mut(&mut self, env: Environment) -> &mut BasicConfig {
        match self.config.get_mut(&env) {
            Some(config) => config,
            None => panic!("set:{} config is missing", env),
        }
    }

    pub fn active_default_from(filename: Option<&Path>) -> Result<EnvConfig, ConfigError> {
        let mut defaults = HashMap::new();
        if let Some(path) = filename {
            defaults.insert(Development, BasicConfig::from(Development, &path)?);
            defaults.insert(Staging, BasicConfig::from(Staging, &path)?);
            defaults.insert(Production, BasicConfig::from(Production, &path)?);
        } else {
            defaults.insert(Development, BasicConfig::default(Development));
            defaults.insert(Staging, BasicConfig::default(Staging));
            defaults.insert(Production, BasicConfig::default(Production));
        }

        let config = EnvConfig {
            active_env: Environment::active()?,
            config: defaults,
        };

        Ok(config)
    }

    pub fn active() -> Result<BasicConfig, ConfigError> {
        Ok(BasicConfig::new(Environment::active()?))
    }

    fn parse<P: AsRef<Path>>(src: String, filename: P) -> Result<EnvConfig, ConfigError> {
        let path = filename.as_ref().to_path_buf();
        let table = match src.parse::<toml::Value>() {
            Ok(toml::Value::Table(table)) => table,
            Ok(value) => {
                let err = format!("expected a table, found {}", value.type_str());
                return Err(ConfigError::ParseError(src, path, err, Some((1, 1))));
            }
            Err(e) => {
                return Err(ConfigError::ParseError(
                    src,
                    path,
                    e.to_string(),
                    Some((2, 2)),
                ))
            }
        };

        // Create a config with the defaults; set the env to the active one.
        let config = EnvConfig::active_default_from(Some(filename.as_ref()))?;

        // Parse the values from the TOML file.
        for (entry, value) in table {
            // Each environment must be a table.
            let _kv_pairs = match value.as_table() {
                Some(table) => table,
                None => {
                    return Err(ConfigError::BadType(
                        entry,
                        "a table",
                        value.type_str(),
                        Some(path.clone()),
                    ))
                }
            };
        }

        Ok(config)
    }

    // 读取配置
    pub fn read_config() -> Result<EnvConfig, ConfigError> {
        // 查找
        let file = EnvConfig::find()?;

        let mut handle = File::open(&file).map_err(|_| ConfigError::IoError)?;

        let mut contents = String::new();
        handle
            .read_to_string(&mut contents)
            .map_err(|_| ConfigError::IoError)?;

        EnvConfig::parse(contents, &file)
    }
}
