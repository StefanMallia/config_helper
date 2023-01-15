# config_loader
A helper library for using the config crate.

E.g.
### appconfig.toml
``` yaml
variable_key="variable_value"
variable_list=["value1", "value2"]
```

### main.rs
``` rust
pub fn main()
{
    let config_path = "appconfig.toml";
    let config_loader: config_loader::ConfigLoader = config_loader::ConfigLoader::new(config_path);

    let value: String = config_loader.get_string("variable_key").unwrap();
    println!("{}", value);

    let variable_list: Vec<String> = config_loader.get_vec("variable_list").unwrap();
    println!("{:?}", variable_list);
}
```

If the config file is not in the working directory, the loader will continue looking for this file in parent directories. This is for the convenience of keeping all configurations in a single file in the root directory of a project when that project contains multiple sub-projects.


## Sub-configurations
Sub-configurations can be used such that they can be passed into a module without that module needing to know the configuration key of the parent. Multiple configuration levels can be specified by using a '.' seperator in the key of the configuration.

### appconfig.toml
``` yaml
variable_key="variable_value_for_project"

[example_service.service_sub_config]
variable_key="variable_value_for_service"
```

### main.rs
``` rust
pub fn main()
{
    let config_path = "appconfig.toml";
    let config_loader: config_loader::ConfigLoader = config_loader::ConfigLoader::new(config_path);

    let sub_config: config_loader::ConfigLoader = config_loader.get_sub_config("example_service");
    let value: String = sub_config.get_string("service_sub_config.variable_key").unwrap();
    println!("{}", value);
}

