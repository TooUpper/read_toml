fn main() {
    let config = read_toml::EnvConfig::read_config();
    println!("{:#?}", config);
}
