pub struct ConfigHelper
{
    settings: config::Config,
}

impl ConfigHelper
{
    pub fn new(file_name: &str) -> ConfigHelper
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
      match settings.merge(config::File::with_name(file_path.join(file_name).to_str().unwrap()))
      {
        Ok(_result) => 
        {          
          println!("Config loaded: {}", file_path.join(file_name).to_str().unwrap());
        },
        Err(_err) => 
        {
            println!("{} {}", _err, file_path.to_str().unwrap());
        }  
      }
      ConfigHelper{settings: settings}
    }

    pub fn get_array(&self, key: &str) -> Result<Vec<String>, config::ConfigError>
    {
      match self.settings.get_array(key)
      {
        Ok(array) => Ok(array.iter().map(|x| x.clone().into_str().unwrap()).collect()),
        Err(e) => Err(e)
      }            
    }

    pub fn get_value(&self, key: &str) -> Result<String, config::ConfigError>
    {
      self.settings.get_str(key)
    }
}


#[cfg(test)]
mod tests
{
    use super::*;
    use std::fs::{File, create_dir_all, remove_dir_all};
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
        let config_helper: ConfigHelper = ConfigHelper::new(&[&dir_name, "/config.toml"].join(""));
        let values_vec: Vec<String> = config_helper.get_array("key1").unwrap();

        //asserts
        assert_eq!(vec!["testvalue1", "testvalue2", "testvalue3", "testvalue4", "testvalue5", "testvalue6", "testvalue7", "testvalue8", "testvalue9"], values_vec);


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
        let config_helper: ConfigHelper = ConfigHelper::new(&[&dir_name, "/config.toml"].join(""));
        let values_vec: Vec<String> = config_helper.get_array("key1").unwrap();

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
        let config_helper: ConfigHelper = ConfigHelper::new(&[&dir_name, "/config.toml"].join(""));
        let value: String = config_helper.get_value("key1").unwrap();


        //asserts
        assert_eq!("testvalue", value);

        //cleanup
        remove_dir_all(dir_name).unwrap();
    }
}
