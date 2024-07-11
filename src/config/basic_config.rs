use crate::config::error::ConfigError;
use crate::environment::Environment;
use crate::environment::Environment::{Development, Production, Staging};
use crate::Path;
use crate::PathBuf;

#[derive(Debug, PartialEq)]
pub struct Database {
    pub(crate) adapter: String,
    pub(crate) db_name: String,
    pub(crate) pool: u32,
}

#[doc(hidden)]
#[derive(Debug)]
pub struct BasicConfig {
    // 该字段用于匹配环境
    pub environment: Environment,
    pub address: String,
    pub port: u16,
    // Option 表示该项是可选的，可以为null
    pub database: Option<Database>,
    // 进程数
    pub workers: Option<u16>,
    pub(crate) config_file_path: Option<PathBuf>,
    pub(crate) root_path: Option<PathBuf>,
}

impl BasicConfig {
    pub fn new(env: Environment) -> Self {
        Self::default(env)
    }

    pub(crate) fn default(env: Environment) -> Self {
        let default_workers = (num_cpus::get() * 2) as u16;

        let default_config = BasicConfig {
            environment: Development,
            address: "localhost".to_string(),
            port: 8000,
            database: None,
            workers: Some(default_workers),
            config_file_path: None,
            root_path: None,
        };

        match env {
            Development => BasicConfig {
                environment: Development,
                ..default_config
            },

            Staging => BasicConfig {
                environment: Staging,
                address: "0.0.0.0".to_string(),
                ..default_config
            },

            Production => BasicConfig {
                environment: Production,
                address: "0.0.0.0".to_string(),
                ..default_config
            },
        }
    }

    pub fn set_root<P: AsRef<Path>>(&mut self, path: P) {
        self.root_path = Some(path.as_ref().into());
    }

    pub(crate) fn from<P>(env: Environment, path: P) -> Result<Self, ConfigError>
    where
        P: AsRef<Path>,
    {
        let mut config = BasicConfig::default(env);

        let config_file_path = path.as_ref().to_path_buf();
        // 如果config_file_path.parent()返回的是Some(PathBuf)
        if let Some(parent) = config_file_path.parent() {
            config.set_root(parent);
        } else {
            let msg = "Configuration files must be rooted in a directory";
            return Err(ConfigError::BadFilePath(config_file_path.clone(), msg));
        }

        config.config_file_path = Some(config_file_path);
        Ok(config)
    }
}

impl PartialEq for BasicConfig {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address && self.port == other.port && self.workers == other.workers
    }
}
