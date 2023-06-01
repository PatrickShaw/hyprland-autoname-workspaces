use crate::renamer::IconConfig::*;
use crate::renamer::IconStatus::*;
use crate::renamer::{ConfigFile, Renamer};

type Rule = String;
type Icon = String;
type Title = String;
type Class = String;
type ListTitleInClass<'a> = Option<&'a [(regex::Regex, Vec<(regex::Regex, Icon)>)]>;
type ListClass<'a> = Option<&'a [(regex::Regex, Icon)]>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IconConfig {
    Class(Rule, Icon),
    InitialClass(Rule, Icon),
    TitleInClass(Rule, Icon),
    TitleInInitialClass(Rule, Icon),
    InitialTitleInClass(Rule, Icon),
    InitialTitleInInitialClass(Rule, Icon),
    Default(Icon),
}

impl IconConfig {
    pub fn icon(&self) -> Icon {
        let (_, icon) = self.get();
        icon
    }

    pub fn rule(&self) -> Rule {
        let (rule, _) = self.get();
        rule
    }

    pub fn get(&self) -> (Rule, Icon) {
        match &self {
            Default(icon) => ("DEFAULT".to_string(), icon.to_string()),
            Class(rule, icon)
            | InitialClass(rule, icon)
            | TitleInClass(rule, icon)
            | TitleInInitialClass(rule, icon)
            | InitialTitleInClass(rule, icon)
            | InitialTitleInInitialClass(rule, icon) => (rule.to_string(), icon.to_string()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IconStatus {
    Active(IconConfig),
    Inactive(IconConfig),
}

impl IconStatus {
    pub fn icon(&self) -> Icon {
        match self {
            Active(config) | Inactive(config) => config.icon(),
        }
    }

    pub fn rule(&self) -> Rule {
        match self {
            Active(config) | Inactive(config) => config.rule(),
        }
    }
}

impl Renamer {
    fn find_icon_status(
        &self,
        initial_class: &str,
        class: &str,
        initial_title: &str,
        title: &str,
        is_active: bool,
        config: &ConfigFile,
    ) -> Option<IconStatus> {
        let (
            list_initial_title_in_initial_class,
            list_initial_title_in_class,
            list_title_in_initial_class,
            list_title_in_class,
            list_initial_class,
            list_class,
        ) = if is_active {
            (
                &config.initial_title_in_initial_class_active,
                &config.initial_title_in_class_active,
                &config.title_in_initial_class_active,
                &config.title_in_class_active,
                &config.initial_class_active,
                &config.class_active,
            )
        } else {
            (
                &config.initial_title_in_initial_class,
                &config.initial_title_in_class,
                &config.title_in_initial_class,
                &config.title_in_class,
                &config.initial_class,
                &config.class,
            )
        };

        find_icon_helper(
            is_active,
            Some(list_initial_title_in_initial_class),
            None,
            None,
            None,
            Some(initial_class),
            Some(initial_title),
        )
        .or(find_icon_helper(
            is_active,
            Some(list_initial_title_in_class),
            None,
            Some(class),
            None,
            None,
            Some(initial_title),
        )
        .or(find_icon_helper(
            is_active,
            Some(list_title_in_initial_class),
            None,
            None,
            Some(title),
            Some(initial_class),
            None,
        )
        .or(find_icon_helper(
            is_active,
            Some(list_title_in_class),
            None,
            Some(class),
            Some(title),
            None,
            None,
        )
        .or(find_icon_helper(
            is_active,
            None,
            Some(list_initial_class),
            None,
            None,
            Some(class),
            None,
        ))
        .or(find_icon_helper(
            is_active,
            None,
            Some(list_class),
            Some(class),
            None,
            None,
            None,
        )))))
    }

    pub fn parse_icon(
        &self,
        initial_class: Class,
        class: Class,
        initial_title: Title,
        title: Title,
        is_active: bool,
        config: &ConfigFile,
    ) -> IconStatus {
        let icon = self.find_icon_status(
            &initial_class,
            &class,
            &initial_title,
            &title,
            false,
            config,
        );
        let icon_active =
            self.find_icon_status(&initial_class, &class, &initial_title, &title, true, config);

        let icon_default = self
            .find_icon_status("DEFAULT", "DEFAULT", "", "", false, config)
            .unwrap_or(Inactive(Default("no icon".to_string())));

        let icon_default_active = self
            .find_icon_status("DEFAULT", "DEFAULT", "", "", true, config)
            .unwrap_or({
                self.find_icon_status("DEFAULT", "DEFAULT", "", "", false, config)
                    .map(|i| Active(Class(i.rule(), i.icon())))
                    .unwrap_or(Active(Default("no icon".to_string())))
            });

        if is_active {
            icon_active.unwrap_or(match icon {
                Some(i) => i,
                None => icon_default_active,
            })
        } else {
            icon.unwrap_or_else(|| {
                if self.args.verbose {
                    println!("- window: class '{}' need a shiny icon", class);
                }
                icon_default
            })
        }
    }
}

pub fn forge_icon_status(
    is_active: bool,
    rule: String,
    icon: String,
    class: Option<&str>,
    title: Option<&str>,
    initial_class: Option<&str>,
    initial_title: Option<&str>,
) -> IconStatus {
    let icon = match (class, title, initial_class, initial_title) {
        (None, None, None, None) => Default(icon),
        (None, None, Some(_), None) => InitialClass(rule, icon),
        (None, None, Some(_), Some(_)) => InitialTitleInInitialClass(rule, icon),
        (None, Some(_), Some(_), None) => TitleInInitialClass(rule, icon),
        (Some(_), None, None, None) => Class(rule, icon),
        (Some(_), None, None, Some(_)) => InitialTitleInClass(rule, icon),
        (Some(_), Some(_), None, None) => TitleInClass(rule, icon),
        (_, _, _, _) => unreachable!(),
    };
    if is_active {
        Active(icon)
    } else {
        Inactive(icon)
    }
}

fn find_icon_helper(
    is_active: bool,
    list_title_in_class: ListTitleInClass,
    list_class: ListClass,
    class: Option<&str>,
    title: Option<&str>,
    initial_class: Option<&str>,
    initial_title: Option<&str>,
) -> Option<IconStatus> {
    match (list_class, list_title_in_class) {
        (Some(list), None) => list
            .iter()
            .find(|(rule, _)| {
                let m = match (class, initial_class) {
                    (Some(m), None) | (None, Some(m)) => m,
                    (Some(_), _) | (None, None) => unreachable!(),
                };
                rule.is_match(m)
            })
            .map(|(rule, icon)| {
                forge_icon_status(
                    is_active,
                    rule.to_string(),
                    icon.to_string(),
                    class,
                    title,
                    initial_class,
                    initial_title,
                )
            }),
        (None, Some(list)) => list
            .iter()
            .find(|(re_class, _)| {
                let m = match (class, initial_class) {
                    (Some(m), None) | (None, Some(m)) => m,
                    (_, _) => unreachable!(),
                };
                re_class.is_match(m)
            })
            .and_then(|(_, title_icon)| {
                title_icon
                    .iter()
                    .find(|(rule, _)| {
                        let m = match (title, initial_title) {
                            (Some(m), None) | (None, Some(m)) => m,
                            (_, _) => unreachable!(),
                        };
                        rule.is_match(m)
                    })
                    .map(|(rule, icon)| {
                        forge_icon_status(
                            is_active,
                            rule.to_string(),
                            icon.to_string(),
                            class,
                            title,
                            initial_class,
                            initial_title,
                        )
                    })
            }),
        (None, None) | (Some(_), Some(_)) => unreachable!(),
    }
}
