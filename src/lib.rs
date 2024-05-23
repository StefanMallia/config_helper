use serde::Deserialize;

pub struct ConfigLoader {
    settings: config::Config,
}

impl ConfigLoader {
    pub fn new(file_name: &str) -> ConfigLoader {
        let mut settings: config::Config = config::Config::new();
        let mut file_path = std::env::current_dir().unwrap();

        loop {
            let appconfig_path = file_path.join(file_name);
            if appconfig_path.exists() {
                break;
            }
            file_path = match file_path.parent() {
                Some(path) => path.to_path_buf(),
                None => panic!("{} not found in any parent directory.", file_name),
            }
        }
        match settings.merge(config::File::with_name(
            file_path.join(file_name).to_str().unwrap(),
        )) {
            Ok(_result) => {
                println!(
                    "Config loaded: {}",
                    file_path.join(file_name).to_str().unwrap()
                );
            }
            Err(_err) => {
                println!("{} {}", _err, file_path.to_str().unwrap());
            }
        }
        match settings.merge(config::Environment::new()) {
            Ok(_result) => {}
            Err(_err) => {
                println!("{}", _err);
            }
        }
        ConfigLoader { settings: settings }
    }

    pub fn get_vec(&self, key: &str) -> Result<Vec<String>, config::ConfigError> {
        match self.settings.get_array(&key) {
            Ok(array) => Ok(array
                .iter()
                .map(|x| x.clone().into_str().unwrap())
                .collect()),
            Err(e) => Err(e),
        }
    }

    pub fn get_string(&self, key: &str) -> Result<String, config::ConfigError> {
        self.settings.get_str(&key)
    }

    pub fn get_int(&self, key: &str) -> Result<i64, config::ConfigError> {
        self.settings.get_int(&key)
    }

    pub fn get_float(&self, key: &str) -> Result<f64, config::ConfigError> {
        self.settings.get_float(&key)
    }

    pub fn get_sub_config(&self, level_key: &str) -> ConfigLoader {
        let hash_map = self.settings.get_table(level_key).unwrap().to_owned();
        let mut config = config::Config::new();
        for (key, value) in hash_map {
            config.set(&key, value).unwrap();
        }
        ConfigLoader { settings: config }
    }

    pub fn deserialize<'de, T: Deserialize<'de>>(&self) -> Result<T, config::ConfigError> {
        T::deserialize(self.settings.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{create_dir_all, remove_dir_all, File};
    use std::io::prelude::*;

    #[test]
    fn test_array_settings() {
        //setup
        let dir_name = "test_assets_1";
        create_dir_all(dir_name).unwrap();
        let mut buffer = File::create([dir_name, "/config.toml"].join("")).unwrap();
        buffer.write_all("key1 = [\"testvalue1\", \"testvalue2\", \"testvalue3\", \"testvalue4\", \"testvalue5\", \"testvalue6\", \"testvalue7\", \"testvalue8\", \"testvalue9\"]".as_bytes()).unwrap();

        //use function
        let config_loader: ConfigLoader = ConfigLoader::new(&[&dir_name, "/config.toml"].join(""));
        let values_vec: Vec<String> = config_loader.get_vec("key1").unwrap();

        //asserts
        assert_eq!(
            vec![
                "testvalue1",
                "testvalue2",
                "testvalue3",
                "testvalue4",
                "testvalue5",
                "testvalue6",
                "testvalue7",
                "testvalue8",
                "testvalue9"
            ],
            values_vec
        );

        //cleanup
        remove_dir_all(dir_name).unwrap();
    }

    #[test]
    fn test_empty_array_settings() {
        //setup
        let dir_name = "test_assets_2";
        create_dir_all(dir_name).unwrap();
        let mut buffer = File::create([dir_name, "/config.toml"].join("")).unwrap();
        buffer.write_all("key1 = [ ]".as_bytes()).unwrap();

        //use function
        let config_loader: ConfigLoader = ConfigLoader::new(&[&dir_name, "/config.toml"].join(""));
        let values_vec: Vec<String> = config_loader.get_vec("key1").unwrap();

        //asserts
        assert_eq!(std::vec::Vec::<String>::new(), values_vec);

        //cleanup
        remove_dir_all(dir_name).unwrap();
    }

    #[test]
    fn test_string_settings() {
        //setup
        let dir_name = "test_assets_3";
        create_dir_all(dir_name).unwrap();
        let mut buffer = File::create([dir_name, "/config.toml"].join("")).unwrap();
        buffer.write_all("key1 = \"testvalue\"".as_bytes()).unwrap();

        //use function
        let config_loader: ConfigLoader = ConfigLoader::new(&[&dir_name, "/config.toml"].join(""));
        let value: String = config_loader.get_string("key1").unwrap();

        //asserts
        assert_eq!("testvalue", value);

        //cleanup
        remove_dir_all(dir_name).unwrap();
    }

    #[test]
    fn test_int_settings() {
        //setup
        let dir_name = "test_assets_test_int";
        create_dir_all(dir_name).unwrap();
        let mut buffer = File::create([dir_name, "/config.toml"].join("")).unwrap();
        buffer.write_all("key1 = 1423114327654".as_bytes()).unwrap();

        //use function
        let config_loader: ConfigLoader = ConfigLoader::new(&[&dir_name, "/config.toml"].join(""));
        let value: i64 = config_loader.get_int("key1").unwrap();

        //asserts
        assert_eq!(1423114327654, value);

        //cleanup
        remove_dir_all(dir_name).unwrap();
    }

    #[test]
    fn test_float_settings() {
        //setup
        let dir_name = "test_assets_test_float";
        create_dir_all(dir_name).unwrap();
        let mut buffer = File::create([dir_name, "/config.toml"].join("")).unwrap();
        buffer
            .write_all("key1 = 34214123.3214321978".as_bytes())
            .unwrap();

        //use function
        let config_loader: ConfigLoader = ConfigLoader::new(&[&dir_name, "/config.toml"].join(""));
        let value: f64 = config_loader.get_float("key1").unwrap();

        //asserts
        assert_eq!(34214123.3214321978, value);

        //cleanup
        remove_dir_all(dir_name).unwrap();
    }

    #[test]
    fn test_int_to_float_settings() {
        //setup
        let dir_name = "test_assets_test_int_to_float";
        create_dir_all(dir_name).unwrap();
        let mut buffer = File::create([dir_name, "/config.toml"].join("")).unwrap();
        buffer.write_all("key1 = 34214123".as_bytes()).unwrap();

        //use function
        let config_loader: ConfigLoader = ConfigLoader::new(&[&dir_name, "/config.toml"].join(""));
        let value: f64 = config_loader.get_float("key1").unwrap();

        //asserts
        assert_eq!(34214123.0, value);

        //cleanup
        remove_dir_all(dir_name).unwrap();
    }

    #[test]
    fn test_hierarchical_value_settings() {
        //setup
        let dir_name = "test_assets_4";
        create_dir_all(dir_name).unwrap();
        let mut buffer = File::create([dir_name, "/config.toml"].join("")).unwrap();
        buffer
            .write_all("key1 = \"testvalue1\"\n\n[test_assets]\nkey2 = \"testvalue2\"".as_bytes())
            .unwrap();

        //use function
        let config_loader: ConfigLoader = ConfigLoader::new(&[&dir_name, "/config.toml"].join(""));
        let value: String = config_loader.get_string("test_assets.key2").unwrap();

        //asserts
        assert_eq!("testvalue2", value);

        //cleanup
        remove_dir_all(dir_name).unwrap();
    }

    #[test]
    fn test_2_level_hierarchical_value_settings() {
        //setup
        let dir_name = "test_assets_5";
        create_dir_all(dir_name).unwrap();
        let mut buffer = File::create([dir_name, "/config.toml"].join("")).unwrap();
        buffer.write_all("key = \"testvalue1\"\n\n[test_assets]\nkey = \"testvalue2\"\n\n[test_assets.level2]\nkey = \"testvalue3\"".as_bytes()).unwrap();

        //use function
        let config_loader: ConfigLoader = ConfigLoader::new(&[&dir_name, "/config.toml"].join(""));
        let value: String = config_loader.get_string("test_assets.level2.key").unwrap();

        //asserts
        assert_eq!("testvalue3", value);

        //cleanup
        remove_dir_all(dir_name).unwrap();
    }

    #[test]
    fn test_fail_get_hierarchical_key_should_be_at_top_of_document() {
        //setup
        let dir_name = "test_assets_6";
        create_dir_all(dir_name).unwrap();
        let mut buffer = File::create([dir_name, "/config.toml"].join("")).unwrap();
        buffer.write_all("key = \"testvalue1\"\n\n[test_assets]\nkey = \"testvalue2\"\n\ntest_assets.level2.key = \"testvalue3\"".as_bytes()).unwrap();

        //use function
        let config_loader: ConfigLoader = ConfigLoader::new(&[&dir_name, "/config.toml"].join(""));
        let value: Result<String, config::ConfigError> =
            config_loader.get_string("test_assets.level2.key");

        //asserts
        match value
        {
            Ok(_x) => assert!(false, "Get value should not work because the 'test_assets.level2.key' key should be at the top of the document. At its current position, test_assets.test_assets.level2.key should be called."),
            Err(_x) => assert!(true, "")
        }

        //cleanup
        remove_dir_all(dir_name).unwrap();
    }

    #[test]
    fn test_successful_get_hierarchical_key_should_include_all_hierchy_level_keys() {
        //setup
        let dir_name = "test_assets_7";
        create_dir_all(dir_name).unwrap();
        let mut buffer = File::create([dir_name, "/config.toml"].join("")).unwrap();
        buffer.write_all("key = \"testvalue1\"\n\n[test_assets]\nkey = \"testvalue2\"\n\ntest_assets.level2.key = \"testvalue3\"".as_bytes()).unwrap();

        //use function
        let config_loader: ConfigLoader = ConfigLoader::new(&[&dir_name, "/config.toml"].join(""));
        let value: String = config_loader
            .get_string("test_assets.test_assets.level2.key")
            .unwrap();

        //asserts
        assert_eq!("testvalue3", value);

        //cleanup
        remove_dir_all(dir_name).unwrap();
    }

    #[test]
    fn test_successful_get_hierarchical_key_should_be_at_top_of_document() {
        //setup
        let dir_name = "test_assets_8";
        create_dir_all(dir_name).unwrap();
        let mut buffer = File::create([dir_name, "/config.toml"].join("")).unwrap();
        buffer.write_all("test.level2.key = \"testvalue3\"\n\nkey = \"testvalue1\"\n\n[test_assets]\nkey = \"testvalue2\"".as_bytes()).unwrap();

        //use function
        let config_loader: ConfigLoader = ConfigLoader::new(&[&dir_name, "/config.toml"].join(""));
        let value: String = config_loader.get_string("test.level2.key").unwrap();

        //asserts
        assert_eq!("testvalue3", value);

        //cleanup
        remove_dir_all(dir_name).unwrap();
    }

    #[test]
    fn test_get_child_config() {
        //setup
        let dir_name = "test_assets_9";
        create_dir_all(dir_name).unwrap();
        let mut buffer = File::create([dir_name, "/config.toml"].join("")).unwrap();
        buffer.write_all("key = \"testvalue1\"\n\n[test_assets]\nkey = \"testvalue2\"\n\n[test_assets.level2]\nkey = \"testvalue3\"".as_bytes()).unwrap();

        //use function
        let config_loader: ConfigLoader = ConfigLoader::new(&[&dir_name, "/config.toml"].join(""));
        let sub_config: ConfigLoader = config_loader.get_sub_config("test_assets");
        let value: String = sub_config.get_string("level2.key").unwrap();

        //asserts
        assert_eq!("testvalue3", value);

        //cleanup
        remove_dir_all(dir_name).unwrap();
    }

    #[test]
    fn test_get_environment_variable() {
        //setup
        let dir_name = "test_get_environment_variable";
        create_dir_all(dir_name).unwrap();
        _ = File::create([dir_name, "/config.toml"].join("")).unwrap();
        std::env::set_var("key", "value");

        //use function
        let config_loader: ConfigLoader = ConfigLoader::new(&[&dir_name, "/config.toml"].join(""));
        let value: String = config_loader.get_string("key").unwrap();

        //asserts
        assert_eq!("value", value);

        //cleanup
        remove_dir_all(dir_name).unwrap();
    }

    #[derive(Debug, Deserialize)]
    struct AppConfig {
        pub name: String,
        pub server_address: String,
        pub port: u16,
        pub max_connections: Option<usize>,
    }

    #[test]
    fn test_deserialize() {
        //setup
        let dir_name = "test_assets_10";
        create_dir_all(dir_name).unwrap();
        let mut buffer = File::create([dir_name, "/config.toml"].join("")).unwrap();
        buffer
            .write_all(
                "name = \"testvalue1\"\nserver_address = \"12.34.56.78:9000\"\nport=1234"
                    .as_bytes(),
            )
            .unwrap();

        //use function
        let config_loader: ConfigLoader = ConfigLoader::new(&[&dir_name, "/config.toml"].join(""));
        let app_config: AppConfig = config_loader.deserialize().unwrap();

        //asserts
        assert_eq!("testvalue1", app_config.name);
        assert_eq!("12.34.56.78:9000", app_config.server_address);
        assert_eq!(1234, app_config.port);
        assert_eq!(None, app_config.max_connections);

        //cleanup
        remove_dir_all(dir_name).unwrap();
    }

    #[test]
    fn test_deserialize_only_one_sub_config() {
        //setup
        let dir_name = "test_assets_11";
        create_dir_all(dir_name).unwrap();
        let mut buffer = File::create([dir_name, "/config.toml"].join("")).unwrap();
        buffer
            .write_all(
                "[sub_config1]\nname = \"testvalue1\"\nserver_address = \"12.34.56.78:9000\"\nport=1234\n[sub_config2]\nsub_config_test_value=1234"
                    .as_bytes(),
            )
            .unwrap();

        //use function
        let config_loader: ConfigLoader = ConfigLoader::new(&[&dir_name, "/config.toml"].join(""));
        let app_config: AppConfig = (config_loader.get_sub_config("sub_config1"))
            .deserialize()
            .unwrap();

        //asserts
        assert_eq!("testvalue1", app_config.name);
        assert_eq!("12.34.56.78:9000", app_config.server_address);
        assert_eq!(1234, app_config.port);
        assert_eq!(None, app_config.max_connections);

        //cleanup
        remove_dir_all(dir_name).unwrap();
    }

    #[test]
    fn test_deserialize_reference() {
        //setup
        let dir_name = "test_assets_12";
        create_dir_all(dir_name).unwrap();
        let mut buffer = File::create([dir_name, "/config.toml"].join("")).unwrap();
        buffer
            .write_all(
                "[sub_config1]\nname = \"testvalue1\"\nserver_address = \"12.34.56.78:9000\"\nport=1234\n[sub_config2]\nsub_config_test_value=1234"
                    .as_bytes(),
            )
            .unwrap();

        //use function
        let config_loader: ConfigLoader = ConfigLoader::new(&[&dir_name, "/config.toml"].join(""));
        let sub_config = &config_loader.get_sub_config("sub_config1");

        let app_config: AppConfig = sub_config.deserialize().unwrap();

        //asserts
        assert_eq!("testvalue1", app_config.name);
        assert_eq!("12.34.56.78:9000", app_config.server_address);
        assert_eq!(1234, app_config.port);
        assert_eq!(None, app_config.max_connections);

        //cleanup
        remove_dir_all(dir_name).unwrap();
    }
}
