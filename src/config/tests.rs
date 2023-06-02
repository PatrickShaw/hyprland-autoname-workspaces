#[cfg(test)]
mod tests {
    use crate::config::*;
    use std::collections::HashMap;

    #[test]
    fn test_generate_title_config() {
        let mut title_icons_map: HashMap<String, HashMap<String, String>> = HashMap::new();
        let mut inner_map: HashMap<String, String> = HashMap::new();
        inner_map.insert("Title1".to_string(), "Icon1".to_string());
        title_icons_map.insert("Class1".to_string(), inner_map);

        let title_config = generate_title_config(&title_icons_map);

        assert_eq!(title_config.len(), 1);
        assert!(title_config[0].0.is_match("Class1"));
        assert_eq!(title_config[0].1.len(), 1);
        assert!(title_config[0].1[0].0.is_match("Title1"));
        assert_eq!(title_config[0].1[0].1, "Icon1");
    }

    #[test]
    fn test_generate_icon_config() {
        let mut list_class: HashMap<String, String> = HashMap::new();
        list_class.insert("Class1".to_string(), "Icon1".to_string());

        let icons_config = generate_icon_config(&list_class);

        assert_eq!(icons_config.len(), 1);
        assert!(icons_config[0].0.is_match("Class1"));
        assert_eq!(icons_config[0].1, "Icon1");
    }

    #[test]
    fn test_generate_exclude_config() {
        let mut list_exclude: HashMap<String, String> = HashMap::new();
        list_exclude.insert("Class1".to_string(), "Title1".to_string());

        let exclude_config = generate_exclude_config(&list_exclude);

        assert_eq!(exclude_config.len(), 1);
        assert!(exclude_config[0].0.is_match("Class1"));
        assert!(exclude_config[0].1.is_match("Title1"));
    }

    #[test]
    fn test_regex_with_error_logging() {
        let valid_pattern = "Class1";
        let invalid_pattern = "Class1[";

        assert!(regex_with_error_logging(valid_pattern).is_some());
        assert!(regex_with_error_logging(invalid_pattern).is_none());
    }

    #[test]
    fn test_config_new_and_read_again_then_compare_format() {
        let cfg_path = PathBuf::from("/tmp/hyprland-autoname-workspaces-test.toml");
        let config = Config::new(cfg_path.clone(), false, false);
        assert_eq!(config.is_ok(), true);
        let config = config.unwrap().clone();
        assert_eq!(config.cfg_path.clone(), Some(cfg_path.clone()));
        let format = config.config.format.clone();
        let config2 = read_config_file(Some(cfg_path.clone()), false, false).unwrap();
        let format2 = config2.format.clone();
        assert_eq!(format, format2);
    }
}
