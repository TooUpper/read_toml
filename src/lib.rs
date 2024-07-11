mod config;
mod environment;

use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
};

pub use config::EnvConfig;
