pub struct ConfigLoader
{
    current_level: String,
    settings: config::Config
}

impl ConfigLoader
{
    pub fn new(file_name: &str) -> ConfigLoader
    {
        let mut settings: config::Config = config::Config::new();
        let mut file_path = std::env::current_dir().unwrap();

        loop
        {
            let appconfig_path = file_path.join(file_name);
            if appconfig_path.exists()
            {
                break;
            }
            file_path = match file_path.parent()
            {
                Some(path) => path.to_path_buf(),
                None => panic!("{} not found in any parent directory.", file_name)
            }
        }
        match settings.merge(config::File::with_name(
            file_path.join(file_name).to_str().unwrap()))
        {
            Ok(_result) =>
            {
                println!(
                    "Config loaded: {}",
                    file_path.join(file_name).to_str().unwrap()
                );
            }
            Err(_err) =>
            {
                println!("{} {}", _err, file_path.to_str().unwrap());
            }
        }
        ConfigLoader
        {
            current_level: "".to_string(),
            settings: settings
        }
    }

    pub fn get_vec(&self, key: &str) -> Result<Vec<String>, config::ConfigError>
    {
        match self
            .settings
            .get_array(&format!("{}{}", self.current_level, key))
        {
            Ok(array) => Ok(array
                .iter()
                .map(|x| x.clone().into_str().unwrap())
                .collect()),
            Err(e) => Err(e)
        }
    }

    pub fn get_string(&self, key: &str) -> Result<String, config::ConfigError>
    {
        self.settings
            .get_str(&format!("{}{}", self.current_level, key))
    }

    pub fn get_sub_config(self, level_key: &str) -> ConfigLoader
    {
        let sub_level = self.current_level + level_key + ".";
        let sub_config = ConfigLoader
        {
            current_level: sub_level.to_string(),
            settings: self.settings.clone()
        };
        sub_config
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use std::fs::{create_dir_all, remove_dir_all, File};
    use std::io::prelude::*;

    #[test]
    fn test_array_settings()
    {
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
    fn test_empty_array_settings()
    {
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
    fn test_value_settings()
    {
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
    fn test_hierarchical_value_settings()
    {
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
    fn test_2_level_hierarchical_value_settings()
    {
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
    fn test_fail_get_hierarchical_key_should_be_at_top_of_document()
    {
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
    fn test_successful_get_hierarchical_key_should_include_all_hierchy_level_keys()
    {
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
    fn test_successful_get_hierarchical_key_should_be_at_top_of_document()
    {
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
    fn test_get_child_config()
    {
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
}
