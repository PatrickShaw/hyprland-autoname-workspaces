#[cfg(test)]
mod tests {
    use regex::Regex;

    use crate::renamer::IconConfig::*;
    use crate::renamer::IconStatus::*;
    use crate::renamer::*;

    #[test]
    fn test_app_client_partial_eq() {
        let client1 = AppClient {
            initial_class: "kitty".to_string(),
            class: "kitty".to_string(),
            title: "~".to_string(),
            is_active: false,
            is_fullscreen: false,
            initial_title: "zsh".to_string(),
            matched_rule: Inactive(Class("(kitty|alacritty)".to_string(), "term".to_string())),
            is_dedup_inactive_fullscreen: false,
        };

        let client2 = AppClient {
            initial_class: "alacritty".to_string(),
            class: "alacritty".to_string(),
            title: "xplr".to_string(),
            initial_title: "zsh".to_string(),
            is_active: false,
            is_fullscreen: false,
            matched_rule: Inactive(Class("(kitty|alacritty)".to_string(), "term".to_string())),
            is_dedup_inactive_fullscreen: false,
        };

        let client3 = AppClient {
            initial_class: "kitty".to_string(),
            class: "kitty".to_string(),
            title: "".to_string(),
            initial_title: "zsh".to_string(),
            is_active: true,
            is_fullscreen: false,
            matched_rule: Active(Class("(kitty|alacritty)".to_string(), "term".to_string())),
            is_dedup_inactive_fullscreen: false,
        };

        let client4 = AppClient {
            initial_class: "alacritty".to_string(),
            class: "alacritty".to_string(),
            title: "".to_string(),
            initial_title: "zsh".to_string(),
            is_active: false,
            is_fullscreen: true,
            matched_rule: Inactive(Class("(kitty|alacritty)".to_string(), "term".to_string())),
            is_dedup_inactive_fullscreen: false,
        };

        let client5 = AppClient {
            initial_class: "kitty".to_string(),
            class: "kitty".to_string(),
            title: "".to_string(),
            initial_title: "zsh".to_string(),
            is_active: false,
            is_fullscreen: true,
            matched_rule: Inactive(Class("(kitty|alacritty)".to_string(), "term".to_string())),
            is_dedup_inactive_fullscreen: false,
        };

        let client6 = AppClient {
            initial_class: "alacritty".to_string(),
            class: "alacritty".to_string(),
            title: "".to_string(),
            initial_title: "zsh".to_string(),
            is_active: false,
            is_fullscreen: false,
            matched_rule: Inactive(Class("alacritty".to_string(), "term".to_string())),
            is_dedup_inactive_fullscreen: false,
        };

        assert_eq!(client1 == client2, true);
        assert_eq!(client4 == client5, true);
        assert_eq!(client1 == client4, false);
        assert_eq!(client1 == client3, false);
        assert_eq!(client5 == client6, false);
    }

    #[test]
    fn test_dedup_kitty_and_alacritty_if_one_regex() {
        let mut config = crate::config::read_config_file(None, false, false).unwrap();
        config
            .class
            .push((Regex::new("(kitty|alacritty)").unwrap(), "term".to_string()));

        config.format.dedup = true;
        config.format.client_dup = "{icon}{counter}".to_string();

        let renamer = Renamer::new(
            Config {
                cfg_path: None,
                config: config.clone(),
            },
            Args {
                verbose: false,
                debug: false,
                config: None,
                dump: false,
                migrate_config: false,
            },
        );

        let expected = [(1, "term5".to_string())].into_iter().collect();

        let actual = renamer.generate_workspaces_string(
            vec![AppWorkspace {
                id: 1,
                clients: vec![
                    AppClient {
                        initial_class: "kitty".to_string(),
                        class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            false,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        class: "kitty".to_string(),
                        initial_class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            false,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        initial_class: "alacritty".to_string(),
                        class: "alacritty".to_string(),
                        title: "alacritty".to_string(),
                        initial_title: "alacritty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "alacritty".to_string(),
                            "alacritty".to_string(),
                            "alacritty".to_string(),
                            "alacritty".to_string(),
                            false,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        class: "alacritty".to_string(),
                        initial_class: "alacritty".to_string(),
                        title: "alacritty".to_string(),
                        initial_title: "alacritty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "alacritty".to_string(),
                            "alacritty".to_string(),
                            "alacritty".to_string(),
                            "alacritty".to_string(),
                            false,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        initial_class: "alacritty".to_string(),
                        class: "alacritty".to_string(),
                        title: "alacritty".to_string(),
                        initial_title: "alacritty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "alacritty".to_string(),
                            "alacritty".to_string(),
                            "alacritty".to_string(),
                            "alacritty".to_string(),
                            false,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                ],
            }],
            &config,
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_icon_initial_title_and_initial_title_active() {
        let mut config = crate::config::read_config_file(None, false, false).unwrap();
        config
            .class
            .push((Regex::new("kitty").unwrap(), "term".to_string()));

        config
            .class
            .push((Regex::new("alacritty").unwrap(), "term".to_string()));

        config.initial_title_in_class.push((
            Regex::new("(kitty|alacritty)").unwrap(),
            vec![(Regex::new("zsh").unwrap(), "Zsh".to_string())],
        ));

        config.initial_title_in_class_active.push((
            Regex::new("alacritty").unwrap(),
            vec![(Regex::new("zsh").unwrap(), "#Zsh#".to_string())],
        ));

        config.format.client_dup = "{icon}{counter}".to_string();

        let renamer = Renamer::new(
            Config {
                cfg_path: None,
                config: config.clone(),
            },
            Args {
                verbose: false,
                debug: false,
                config: None,
                dump: false,
                migrate_config: false,
            },
        );

        let expected = [(1, "Zsh #Zsh# *Zsh*".to_string())].into_iter().collect();

        let actual = renamer.generate_workspaces_string(
            vec![AppWorkspace {
                id: 1,
                clients: vec![
                    AppClient {
                        initial_class: "alacritty".to_string(),
                        class: "alacritty".to_string(),
                        title: "alacritty".to_string(),
                        initial_title: "zsh".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "alacritty".to_string(),
                            "alacritty".to_string(),
                            "zsh".to_string(),
                            "alacritty".to_string(),
                            false,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        initial_class: "alacritty".to_string(),
                        class: "alacritty".to_string(),
                        title: "alacritty".to_string(),
                        initial_title: "zsh".to_string(),
                        is_active: true,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "alacritty".to_string(),
                            "alacritty".to_string(),
                            "zsh".to_string(),
                            "alacritty".to_string(),
                            true,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        initial_class: "kitty".to_string(),
                        class: "kitty".to_string(),
                        title: "~".to_string(),
                        initial_title: "zsh".to_string(),
                        is_active: true,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "zsh".to_string(),
                            "~".to_string(),
                            true,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                ],
            }],
            &config,
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_dedup_kitty_and_alacritty_if_two_regex() {
        let mut config = crate::config::read_config_file(None, false, false).unwrap();
        config
            .class
            .push((Regex::new("kitty").unwrap(), "term".to_string()));

        config
            .class
            .push((Regex::new("alacritty").unwrap(), "term".to_string()));

        config.format.dedup = true;
        config.format.client_dup = "{icon}{counter}".to_string();

        let renamer = Renamer::new(
            Config {
                cfg_path: None,
                config: config.clone(),
            },
            Args {
                verbose: false,
                debug: false,
                config: None,
                dump: false,
                migrate_config: false,
            },
        );

        let expected = [(1, "term2 term3".to_string())].into_iter().collect();

        let actual = renamer.generate_workspaces_string(
            vec![AppWorkspace {
                id: 1,
                clients: vec![
                    AppClient {
                        class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_class: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            false,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        class: "kitty".to_string(),
                        initial_class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            false,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        class: "alacritty".to_string(),
                        initial_class: "alacritty".to_string(),
                        title: "alacritty".to_string(),
                        initial_title: "alacritty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "alacritty".to_string(),
                            "alacritty".to_string(),
                            "alacritty".to_string(),
                            "alacritty".to_string(),
                            false,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        class: "alacritty".to_string(),
                        initial_class: "alacritty".to_string(),
                        title: "alacritty".to_string(),
                        initial_title: "alacritty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "alacritty".to_string(),
                            "alacritty".to_string(),
                            "alacritty".to_string(),
                            "alacritty".to_string(),
                            false,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        initial_class: "alacritty".to_string(),
                        class: "alacritty".to_string(),
                        title: "alacritty".to_string(),
                        initial_title: "alacritty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "alacritty".to_string(),
                            "alacritty".to_string(),
                            "alacritty".to_string(),
                            "alacritty".to_string(),
                            false,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                ],
            }],
            &config,
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_to_superscript() {
        let input = 1234567890;
        let expected = "¹²³⁴⁵⁶⁷⁸⁹⁰";
        let output = to_superscript(input);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_no_dedup_no_focus_no_fullscreen_one_workspace() {
        let mut config = crate::config::read_config_file(None, false, false).unwrap();
        config
            .class
            .push((Regex::new("kitty").unwrap(), "term".to_string()));

        let renamer = Renamer::new(
            Config {
                cfg_path: None,
                config: config.clone(),
            },
            Args {
                verbose: false,
                debug: false,
                config: None,
                dump: false,
                migrate_config: false,
            },
        );

        let expected = [(1, "term term term term term".to_string())]
            .into_iter()
            .collect();

        let actual = renamer.generate_workspaces_string(
            vec![AppWorkspace {
                id: 1,
                clients: vec![
                    AppClient {
                        initial_class: "kitty".to_string(),
                        class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            false,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        class: "kitty".to_string(),
                        initial_class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            false,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        class: "kitty".to_string(),
                        initial_class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            false,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        initial_class: "kitty".to_string(),
                        class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            false,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        class: "kitty".to_string(),
                        initial_class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            false,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                ],
            }],
            &config,
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_no_dedup_focus_no_fullscreen_one_workspace_middle() {
        let mut config = crate::config::read_config_file(None, false, false).unwrap();
        config
            .class
            .push((Regex::new("kitty").unwrap(), "term".to_string()));
        config.format.client_active = "*{icon}*".to_string();

        let renamer = Renamer::new(
            Config {
                cfg_path: None,
                config: config.clone(),
            },
            Args {
                verbose: false,
                debug: false,
                dump: false,
                config: None,
                migrate_config: false,
            },
        );

        let expected = [(1, "term term *term* term term".to_string())]
            .into_iter()
            .collect();

        let actual = renamer.generate_workspaces_string(
            vec![AppWorkspace {
                id: 1,
                clients: vec![
                    AppClient {
                        initial_class: "kitty".to_string(),
                        class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            false,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        initial_class: "kitty".to_string(),
                        class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            false,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        class: "kitty".to_string(),
                        initial_class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: true,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            true,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        class: "kitty".to_string(),
                        initial_class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            false,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        class: "kitty".to_string(),
                        initial_class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            false,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                ],
            }],
            &config,
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_no_dedup_no_focus_fullscreen_one_workspace_middle() {
        let mut config = crate::config::read_config_file(None, false, false).unwrap();
        config
            .class
            .push((Regex::new("kitty").unwrap(), "term".to_string()));
        config.format.client_active = "*{icon}*".to_string();
        config.format.client_fullscreen = "[{icon}]".to_string();

        let renamer = Renamer::new(
            Config {
                cfg_path: None,
                config: config.clone(),
            },
            Args {
                verbose: false,
                debug: false,
                dump: false,
                migrate_config: false,
                config: None,
            },
        );

        let expected = [(1, "term term [term] term term".to_string())]
            .into_iter()
            .collect();

        let actual = renamer.generate_workspaces_string(
            vec![AppWorkspace {
                id: 1,
                clients: vec![
                    AppClient {
                        initial_class: "kitty".to_string(),
                        class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            false,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        class: "kitty".to_string(),
                        initial_class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            false,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        class: "kitty".to_string(),
                        initial_class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: false,
                        is_fullscreen: true,
                        matched_rule: renamer.parse_icon(
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            false,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        class: "kitty".to_string(),
                        initial_class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            false,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        initial_class: "kitty".to_string(),
                        class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            false,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                ],
            }],
            &config,
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_no_dedup_focus_fullscreen_one_workspace_middle() {
        let mut config = crate::config::read_config_file(None, false, false).unwrap();
        config
            .class
            .push((Regex::new("kitty").unwrap(), "term".to_string()));
        config.format.client_active = "*{icon}*".to_string();
        config.format.client_fullscreen = "[{icon}]".to_string();

        let renamer = Renamer::new(
            Config {
                cfg_path: None,
                config: config.clone(),
            },
            Args {
                verbose: false,
                debug: false,
                dump: false,
                migrate_config: false,
                config: None,
            },
        );

        let expected = [(1, "term term [*term*] term term".to_string())]
            .into_iter()
            .collect();

        let actual = renamer.generate_workspaces_string(
            vec![AppWorkspace {
                id: 1,
                clients: vec![
                    AppClient {
                        class: "kitty".to_string(),
                        initial_class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            false,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        class: "kitty".to_string(),
                        initial_class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            false,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        class: "kitty".to_string(),
                        initial_class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: true,
                        is_fullscreen: true,
                        matched_rule: renamer.parse_icon(
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            true,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        class: "kitty".to_string(),
                        initial_class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            false,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        class: "kitty".to_string(),
                        initial_class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            false,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                ],
            }],
            &config,
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_dedup_no_focus_no_fullscreen_one_workspace() {
        let mut config = crate::config::read_config_file(None, false, false).unwrap();
        config
            .class
            .push((Regex::new("kitty").unwrap(), "term".to_string()));
        config.format.dedup = true;
        config.format.client_dup = "{icon}{counter}".to_string();

        let renamer = Renamer::new(
            Config {
                cfg_path: None,
                config: config.clone(),
            },
            Args {
                verbose: false,
                debug: false,
                dump: false,
                migrate_config: false,
                config: None,
            },
        );

        let expected = [(1, "term5".to_string())].into_iter().collect();

        let actual = renamer.generate_workspaces_string(
            vec![AppWorkspace {
                id: 1,
                clients: vec![
                    AppClient {
                        initial_class: "kitty".to_string(),
                        class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: Inactive(Class("kitty".to_string(), "term".to_string())),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        initial_class: "kitty".to_string(),
                        class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: Inactive(Class("kitty".to_string(), "term".to_string())),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        initial_class: "kitty".to_string(),
                        class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: Inactive(Class("kitty".to_string(), "term".to_string())),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        initial_class: "kitty".to_string(),
                        class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: Inactive(Class("kitty".to_string(), "term".to_string())),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        initial_class: "kitty".to_string(),
                        class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: Inactive(Class("kitty".to_string(), "term".to_string())),
                        is_dedup_inactive_fullscreen: false,
                    },
                ],
            }],
            &config,
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_dedup_focus_no_fullscreen_one_workspace_middle() {
        let mut config = crate::config::read_config_file(None, false, false).unwrap();
        config
            .class
            .push((Regex::new("kitty").unwrap(), "term".to_string()));

        config.format.dedup = true;
        config.format.client_dup = "{icon}{counter}".to_string();
        config.format.client_active = "*{icon}*".to_string();
        config.format.client_dup_active = "{icon}{counter_unfocused}".to_string();

        let renamer = Renamer::new(
            Config {
                cfg_path: None,
                config: config.clone(),
            },
            Args {
                verbose: false,
                debug: false,
                dump: false,
                migrate_config: false,
                config: None,
            },
        );

        let expected = [(1, "*term* term4".to_string())].into_iter().collect();

        let actual = renamer.generate_workspaces_string(
            vec![AppWorkspace {
                id: 1,
                clients: vec![
                    AppClient {
                        class: "kitty".to_string(),
                        initial_class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            false,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        class: "kitty".to_string(),
                        initial_class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            false,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        initial_class: "kitty".to_string(),
                        class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: true,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            true,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        initial_class: "kitty".to_string(),
                        class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            false,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        class: "kitty".to_string(),
                        initial_class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            false,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                ],
            }],
            &config,
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_dedup_no_focus_fullscreen_one_workspace_middle() {
        let mut config = crate::config::read_config_file(None, false, false).unwrap();
        config
            .class
            .push((Regex::new("kitty").unwrap(), "term".to_string()));

        config.format.dedup = true;
        config.format.client_dup = "{icon}{counter}".to_string();
        config.format.client_dup_fullscreen =
            "[{icon}]{delim}{icon}{counter_unfocused_sup}".to_string();

        let renamer = Renamer::new(
            Config {
                cfg_path: None,
                config: config.clone(),
            },
            Args {
                verbose: false,
                debug: false,
                config: None,
                dump: false,
                migrate_config: false,
            },
        );

        let expected = [(1, "[term] term4".to_string())].into_iter().collect();

        let actual = renamer.generate_workspaces_string(
            vec![AppWorkspace {
                id: 1,
                clients: vec![
                    AppClient {
                        class: "kitty".to_string(),
                        initial_class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            false,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        class: "kitty".to_string(),
                        initial_class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            false,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        class: "kitty".to_string(),
                        initial_class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: false,
                        is_fullscreen: true,
                        matched_rule: renamer.parse_icon(
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            false,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        class: "kitty".to_string(),
                        initial_class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            false,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        class: "kitty".to_string(),
                        initial_class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            false,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                ],
            }],
            &config,
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_dedup_focus_fullscreen_one_workspace_middle() {
        let mut config = crate::config::read_config_file(None, false, false).unwrap();
        config
            .class
            .push((Regex::new("kitty").unwrap(), "term".to_string()));
        config.format.dedup = true;
        config.format.client = "{icon}".to_string();
        config.format.client_active = "*{icon}*".to_string();
        config.format.client_fullscreen = "[{icon}]".to_string();
        config.format.client_dup = "{icon}{counter}".to_string();
        config.format.client_dup_fullscreen =
            "[{icon}]{delim}{icon}{counter_unfocused}".to_string();
        config.format.client_dup_active = "*{icon}*{delim}{icon}{counter_unfocused}".to_string();

        let renamer = Renamer::new(
            Config {
                cfg_path: None,
                config: config.clone(),
            },
            Args {
                verbose: false,
                debug: false,
                config: None,
                dump: false,
                migrate_config: false,
            },
        );

        let expected = [(1, "[*term*] term4".to_string())].into_iter().collect();

        let actual = renamer.generate_workspaces_string(
            vec![AppWorkspace {
                id: 1,
                clients: vec![
                    AppClient {
                        class: "kitty".to_string(),
                        initial_class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            false,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        class: "kitty".to_string(),
                        initial_class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            false,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        initial_class: "kitty".to_string(),
                        class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: true,
                        is_fullscreen: true,
                        matched_rule: renamer.parse_icon(
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            true,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        class: "kitty".to_string(),
                        initial_class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            false,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        initial_class: "kitty".to_string(),
                        class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: false,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            false,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                ],
            }],
            &config,
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_default_active_icon() {
        let mut config = crate::config::read_config_file(None, false, false).unwrap();
        config
            .class
            .push((Regex::new("kitty").unwrap(), "k".to_string()));
        config
            .class
            .push((Regex::new("alacritty").unwrap(), "a".to_string()));
        config
            .class
            .push((Regex::new("DEFAULT").unwrap(), "d".to_string()));

        config
            .class_active
            .push((Regex::new("kitty").unwrap(), "KKK".to_string()));
        config
            .class_active
            .push((Regex::new("DEFAULT").unwrap(), "DDD".to_string()));

        config.format.client_active = "*{icon}*".to_string();

        let renamer = Renamer::new(
            Config {
                cfg_path: None,
                config: config.clone(),
            },
            Args {
                verbose: false,
                debug: false,
                config: None,
                dump: false,
                migrate_config: false,
            },
        );

        let expected = [(1, "KKK *a* DDD".to_string())].into_iter().collect();

        let actual = renamer.generate_workspaces_string(
            vec![AppWorkspace {
                id: 1,
                clients: vec![
                    AppClient {
                        initial_class: "kitty".to_string(),
                        class: "kitty".to_string(),
                        title: "kitty".to_string(),
                        initial_title: "kitty".to_string(),
                        is_active: true,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            "kitty".to_string(),
                            true,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        class: "alacritty".to_string(),
                        initial_class: "alacritty".to_string(),
                        title: "alacritty".to_string(),
                        initial_title: "alacritty".to_string(),
                        is_active: true,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "alacritty".to_string(),
                            "alacritty".to_string(),
                            "alacritty".to_string(),
                            "alacritty".to_string(),
                            true,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                    AppClient {
                        class: "qute".to_string(),
                        initial_class: "qute".to_string(),
                        title: "qute".to_string(),
                        initial_title: "qute".to_string(),
                        is_active: true,
                        is_fullscreen: false,
                        matched_rule: renamer.parse_icon(
                            "qute".to_string(),
                            "qute".to_string(),
                            "qute".to_string(),
                            "qute".to_string(),
                            true,
                            &config,
                        ),
                        is_dedup_inactive_fullscreen: false,
                    },
                ],
            }],
            &config,
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_no_class_but_title_icon() {
        let mut config = crate::config::read_config_file(None, false, false).unwrap();
        config.title_in_class.push((
            Regex::new("^$").unwrap(),
            vec![(Regex::new("(?i)spotify").unwrap(), "spotify".to_string())],
        ));

        let renamer = Renamer::new(
            Config {
                cfg_path: None,
                config: config.clone(),
            },
            Args {
                verbose: false,
                debug: false,
                config: None,
                dump: false,
                migrate_config: false,
            },
        );

        let expected = [(1, "spotify".to_string())].into_iter().collect();

        let actual = renamer.generate_workspaces_string(
            vec![AppWorkspace {
                id: 1,
                clients: vec![AppClient {
                    initial_class: "".to_string(),
                    class: "".to_string(),
                    title: "spotify".to_string(),
                    initial_title: "spotify".to_string(),
                    is_active: false,
                    is_fullscreen: false,
                    matched_rule: renamer.parse_icon(
                        "".to_string(),
                        "".to_string(),
                        "spotify".to_string(),
                        "spotify".to_string(),
                        false,
                        &config,
                    ),
                    is_dedup_inactive_fullscreen: false,
                }],
            }],
            &config,
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_no_default_class_active_fallback_to_class_default() {
        let mut config = crate::config::read_config_file(None, false, false).unwrap();

        config
            .class_active
            .push((Regex::new("DEFAULT").unwrap(), "default active".to_string()));

        let renamer = Renamer::new(
            Config {
                cfg_path: None,
                config: config.clone(),
            },
            Args {
                verbose: false,
                debug: false,
                config: None,
                dump: false,
                migrate_config: false,
            },
        );

        let expected = [(1, "default active".to_string())].into_iter().collect();

        let actual = renamer.generate_workspaces_string(
            vec![AppWorkspace {
                id: 1,
                clients: vec![AppClient {
                    initial_class: "kitty".to_string(),
                    class: "kitty".to_string(),
                    title: "~".to_string(),
                    initial_title: "zsh".to_string(),
                    is_active: true,
                    is_fullscreen: false,
                    matched_rule: renamer.parse_icon(
                        "kitty".to_string(),
                        "kitty".to_string(),
                        "zsh".to_string(),
                        "~".to_string(),
                        true,
                        &config,
                    ),
                    is_dedup_inactive_fullscreen: false,
                }],
            }],
            &config,
        );

        assert_eq!(actual, expected);

        let config = crate::config::read_config_file(None, false, false).unwrap();

        let renamer = Renamer::new(
            Config {
                cfg_path: None,
                config: config.clone(),
            },
            Args {
                verbose: false,
                debug: false,
                config: None,
                dump: false,
                migrate_config: false,
            },
        );

        let actual = renamer.generate_workspaces_string(
            vec![AppWorkspace {
                id: 1,
                clients: vec![AppClient {
                    initial_class: "kitty".to_string(),
                    class: "kitty".to_string(),
                    initial_title: "zsh".to_string(),
                    title: "~".to_string(),
                    is_active: true,
                    is_fullscreen: false,
                    matched_rule: renamer.parse_icon(
                        "kitty".to_string(),
                        "kitty".to_string(),
                        "zsh".to_string(),
                        "~".to_string(),
                        true,
                        &config,
                    ),
                    is_dedup_inactive_fullscreen: false,
                }],
            }],
            &config,
        );

        let expected = [(1, "\u{f059} kitty".to_string())].into_iter().collect();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_initial_title_in_initial_class_combos() {
        let mut config = crate::config::read_config_file(None, false, false).unwrap();

        config
            .class
            .push((Regex::new("kitty").unwrap(), "term0".to_string()));

        config.title_in_class.push((
            Regex::new("kitty").unwrap(),
            vec![(Regex::new("~").unwrap(), "term1".to_string())],
        ));

        config.title_in_initial_class.push((
            Regex::new("kitty").unwrap(),
            vec![(Regex::new("~").unwrap(), "term2".to_string())],
        ));

        let renamer = Renamer::new(
            Config {
                cfg_path: None,
                config: config.clone(),
            },
            Args {
                verbose: false,
                debug: false,
                config: None,
                dump: false,
                migrate_config: false,
            },
        );

        let expected = [(1, "term2".to_string())].into_iter().collect();

        let actual = renamer.generate_workspaces_string(
            vec![AppWorkspace {
                id: 1,
                clients: vec![AppClient {
                    initial_class: "kitty".to_string(),
                    class: "kitty".to_string(),
                    title: "~".to_string(),
                    initial_title: "zsh".to_string(),
                    is_active: false,
                    is_fullscreen: false,
                    is_dedup_inactive_fullscreen: false,
                    matched_rule: renamer.parse_icon(
                        "kitty".to_string(),
                        "kitty".to_string(),
                        "zsh".to_string(),
                        "~".to_string(),
                        false,
                        &config,
                    ),
                }],
            }],
            &config,
        );

        assert_eq!(actual, expected);

        config.initial_title_in_class.push((
            Regex::new("kitty").unwrap(),
            vec![(Regex::new("(?i)zsh").unwrap(), "term3".to_string())],
        ));

        let renamer = Renamer::new(
            Config {
                cfg_path: None,
                config: config.clone(),
            },
            Args {
                verbose: false,
                debug: false,
                config: None,
                dump: false,
                migrate_config: false,
            },
        );

        let actual = renamer.generate_workspaces_string(
            vec![AppWorkspace {
                id: 1,
                clients: vec![AppClient {
                    initial_class: "kitty".to_string(),
                    class: "kitty".to_string(),
                    initial_title: "zsh".to_string(),
                    title: "~".to_string(),
                    is_active: false,
                    is_fullscreen: false,
                    matched_rule: renamer.parse_icon(
                        "kitty".to_string(),
                        "kitty".to_string(),
                        "zsh".to_string(),
                        "~".to_string(),
                        false,
                        &config,
                    ),
                    is_dedup_inactive_fullscreen: false,
                }],
            }],
            &config,
        );

        let expected = [(1, "term3".to_string())].into_iter().collect();

        assert_eq!(actual, expected);

        config.initial_title_in_initial_class.push((
            Regex::new("kitty").unwrap(),
            vec![(Regex::new("(?i)zsh").unwrap(), "term4".to_string())],
        ));

        let renamer = Renamer::new(
            Config {
                cfg_path: None,
                config: config.clone(),
            },
            Args {
                verbose: false,
                debug: false,
                config: None,
                dump: false,
                migrate_config: false,
            },
        );

        let actual = renamer.generate_workspaces_string(
            vec![AppWorkspace {
                id: 1,
                clients: vec![AppClient {
                    initial_class: "kitty".to_string(),
                    class: "kitty".to_string(),
                    initial_title: "zsh".to_string(),
                    title: "~".to_string(),
                    is_active: false,
                    is_fullscreen: false,
                    matched_rule: renamer.parse_icon(
                        "kitty".to_string(),
                        "kitty".to_string(),
                        "zsh".to_string(),
                        "~".to_string(),
                        false,
                        &config,
                    ),
                    is_dedup_inactive_fullscreen: false,
                }],
            }],
            &config,
        );

        let expected = [(1, "term4".to_string())].into_iter().collect();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_regex_capture_support() {
        let mut config = crate::config::read_config_file(None, false, false).unwrap();

        config.title_in_class.push((
            Regex::new("(?i)foot").unwrap(),
            vec![(
                Regex::new("emerge: (.+?/.+?)-.*").unwrap(),
                "test {match1}".to_string(),
            )],
        ));
        config.title_in_class.push((
            Regex::new("(?i)foot").unwrap(),
            vec![(
                Regex::new("pacman: (.+?/.+?)-(.*)").unwrap(),
                "test {match1} test2 {match2}".to_string(),
            )],
        ));
        config.title_in_class_active.push((
            Regex::new("(?i)foot").unwrap(),
            vec![(
                Regex::new("pacman: (.+?/.+?)-(.*)").unwrap(),
                "*#test{match1}#between#{match2}endtest#*".to_string(),
            )],
        ));

        config.format.client_active = "*{icon}*".to_string();

        let renamer = Renamer::new(
            Config {
                cfg_path: None,
                config: config.clone(),
            },
            Args {
                verbose: false,
                debug: false,
                config: None,
                dump: false,
                migrate_config: false,
            },
        );

        let mut expected = [(1, "test (13 of 20) dev-lang/rust".to_string())]
            .into_iter()
            .collect();

        let mut actual = renamer.generate_workspaces_string(
            vec![AppWorkspace {
                id: 1,
                clients: vec![AppClient {
                    initial_class: "foot".to_string(),
                    class: "foot".to_string(),
                    initial_title: "zsh".to_string(),
                    title: "emerge: (13 of 20) dev-lang/rust-1.69.0-r1 Compile:".to_string(),
                    is_active: false,
                    is_fullscreen: false,
                    matched_rule: renamer.parse_icon(
                        "foot".to_string(),
                        "foot".to_string(),
                        "zsh".to_string(),
                        "emerge: (13 of 20) dev-lang/rust-1.69.0-r1 Compile:".to_string(),
                        false,
                        &config,
                    ),
                    is_dedup_inactive_fullscreen: false,
                }],
            }],
            &config,
        );

        assert_eq!(actual, expected);

        expected = [(
            1,
            "*#test(14 of 20) dev-lang/rust#between#1.69.0-r1 Compile:endtest#*".to_string(),
        )]
        .into_iter()
        .collect();

        actual = renamer.generate_workspaces_string(
            vec![AppWorkspace {
                id: 1,
                clients: vec![AppClient {
                    initial_class: "foot".to_string(),
                    class: "foot".to_string(),
                    initial_title: "zsh".to_string(),
                    title: "pacman: (14 of 20) dev-lang/rust-1.69.0-r1 Compile:".to_string(),
                    is_active: true,
                    is_fullscreen: false,
                    matched_rule: renamer.parse_icon(
                        "foot".to_string(),
                        "foot".to_string(),
                        "zsh".to_string(),
                        "pacman: (14 of 20) dev-lang/rust-1.69.0-r1 Compile:".to_string(),
                        true,
                        &config,
                    ),
                    is_dedup_inactive_fullscreen: false,
                }],
            }],
            &config,
        );

        assert_eq!(actual, expected);
    }
}
