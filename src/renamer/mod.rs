mod formatter;
mod icon;
mod tests;

#[macro_use]
mod macros;

use crate::config::{Config, ConfigFile, ConfigFormatRaw};
use crate::params::Args;
use formatter::*;
use hyprland::data::{Client, Clients, Workspace};
use hyprland::dispatch::*;
use hyprland::event_listener::EventListenerMutable as EventListener;
use hyprland::prelude::*;
use hyprland::shared::{Address, WorkspaceType};
use icon::{IconConfig, IconStatus};
use inotify::{Inotify, WatchMask};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub struct Renamer {
    known_workspaces: Mutex<HashSet<i32>>,
    cfg: Mutex<Config>,
    args: Args,
}

#[derive(Eq, Debug)]
pub struct AppClient<'a> {
    class: &'a str,
    title: &'a str,
    //FIXME: I can't understand why clippy
    // see dead code, but for me, my code is not dead!
    #[allow(dead_code)]
    initial_class: &'a str,
    #[allow(dead_code)]
    initial_title: &'a str,
    is_active: bool,
    is_fullscreen: bool,
    is_dedup_inactive_fullscreen: bool,
    matched_rule: IconStatus,
}

impl<'a> PartialEq for AppClient<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.matched_rule == other.matched_rule
            && self.is_active == other.is_active
            && (self.is_dedup_inactive_fullscreen || self.is_fullscreen == other.is_fullscreen)
    }
}

impl AppClient {
    fn new(
        client: &Client,
        is_active: bool,
        is_dedup_inactive_fullscreen: bool,
        matched_rule: IconStatus,
    ) -> Self {
        AppClient {
            is_active,
            class: &client.class,
            title: &client.title,
            initial_class: &client.initial_class,
            initial_title: &client.initial_title,
            is_fullscreen: client.fullscreen,
            is_dedup_inactive_fullscreen,
            matched_rule,
        }
    }
}

impl Renamer {
    pub fn new(cfg: Config, args: Args) -> Arc<Self> {
        Arc::new(Renamer {
            known_workspaces: Mutex::new(HashSet::default()),
            cfg: Mutex::new(cfg),
            args,
        })
    }

    pub fn rename_workspace(&self) -> Result<(), Box<dyn Error + '_>> {
        // Config
        let config = &self.cfg.lock()?.config.clone();

        // Rename active workspace if empty
        rename_empty_workspace(config);

        // Filter clients
        let clients = get_filtered_clients(config);

        // Get the active client
        let active_client = get_active_client();

        // Get workspaces based on open clients
        let workspaces = self.get_workspaces_from_clients(clients, active_client, config)?;

        // Generate workspace strings
        let workspaces_strings = self.generate_workspaces_string(workspaces, config);

        // Render the workspaces
        workspaces_strings
            .iter()
            .for_each(|(&id, clients)| rename_cmd(id, clients, &config.format));

        Ok(())
    }

    fn get_workspaces_from_clients(
        &self,
        clients: Vec<Client>,
        active_client: String,
        config: &ConfigFile,
    ) -> Result<Vec<AppWorkspace>, Box<dyn Error + '_>> {
        let mut workspaces = self
            .known_workspaces
            .lock()?
            .iter()
            .map(|&i| (i, Vec::new()))
            .collect::<HashMap<i32, Vec<&AppClient>>>();

        let is_dedup_inactive_fullscreen = config.format.dedup_inactive_fullscreen;

        for client in clients {
            let workspace_id = client.workspace.id;
            self.known_workspaces.lock()?.insert(workspace_id);
            let is_active = active_client == client.address.to_string();
            workspaces
                .entry(workspace_id)
                .or_insert_with(Vec::new)
                .push({
                    let x = AppClient::new(
                        &client,
                        is_active,
                        is_dedup_inactive_fullscreen,
                        self.parse_icon(&client, is_active, config),
                    );
                    &x
                });
        }

        Ok(workspaces
            .iter()
            .map(|(&id, &clients)| AppWorkspace::new(id, clients))
            .collect())
    }

    pub fn reset_workspaces(&self, config: ConfigFile) -> Result<(), Box<dyn Error + '_>> {
        self.known_workspaces
            .lock()?
            .iter()
            .for_each(|&id| rename_cmd(id, "", &config.format));

        Ok(())
    }

    pub fn start_listeners(self: &Arc<Self>) {
        let mut event_listener = EventListener::new();

        rename_workspace_if!(
            self,
            event_listener,
            add_window_open_handler,
            add_window_close_handler,
            add_window_moved_handler,
            add_active_window_change_handler,
            add_workspace_added_handler,
            add_workspace_moved_handler,
            add_workspace_change_handler,
            add_fullscreen_state_change_handler
        );

        let this = self.clone();
        event_listener.add_workspace_destroy_handler(move |wt, _| {
            _ = this.rename_workspace();
            _ = this.remove_workspace(wt);
        });

        _ = event_listener.start_listener();
    }

    pub fn watch_config_changes(
        &self,
        cfg_path: Option<PathBuf>,
    ) -> Result<(), Box<dyn Error + '_>> {
        match &cfg_path {
            Some(cfg_path) => {
                loop {
                    // Watch for modify events.
                    let mut notify = Inotify::init()?;

                    notify.add_watch(cfg_path, WatchMask::MODIFY)?;
                    let mut buffer = [0; 1024];
                    notify.read_events_blocking(&mut buffer)?.last();

                    println!("Reloading config !");
                    // Clojure to force quick release of lock
                    {
                        match Config::new(cfg_path.clone(), false, false) {
                            Ok(config) => self.cfg.lock()?.config = config.config,
                            Err(err) => println!("Unable to reload config: {err:?}"),
                        }
                    }

                    // Handle event
                    // Run on window events
                    _ = self.rename_workspace();
                }
            }
            None => Ok(()),
        }
    }

    fn remove_workspace(&self, wt: WorkspaceType) -> Result<bool, Box<dyn Error + '_>> {
        Ok(match wt {
            WorkspaceType::Regular(x) => self.known_workspaces.lock()?.remove(&x.parse::<i32>()?),
            WorkspaceType::Special(_) => false,
        })
    }
}

fn rename_empty_workspace(config: &ConfigFile) {
    let config_format = &config.format;

    _ = Workspace::get_active().map(|workspace| {
        if workspace.windows == 0 {
            rename_cmd(workspace.id, "", config_format);
        }
    });
}

fn rename_cmd(id: i32, clients: &str, config_format: &ConfigFormatRaw) {
    let workspace_fmt = &config_format.workspace.to_string();
    let workspace_empty_fmt = &config_format.workspace_empty.to_string();
    let id_two_digits = format!("{:02}", id);
    let mut vars = HashMap::from([
        ("id".to_string(), id.to_string()),
        ("id_long".to_string(), id_two_digits),
        ("delim".to_string(), config_format.delim.to_string()),
    ]);
    vars.insert("clients".to_string(), clients.to_string());
    let workspace = if !clients.is_empty() {
        formatter(workspace_fmt, &vars)
    } else {
        formatter(workspace_empty_fmt, &vars)
    };

    let _ = hyprland::dispatch!(RenameWorkspace, id, Some(workspace.trim()));
}

fn get_filtered_clients(config: &ConfigFile) -> Vec<Client> {
    let binding = Clients::get().unwrap();
    let config_exclude = &config.exclude;

    binding
        .filter(|client| !client.class.is_empty())
        .filter(|client| {
            !config_exclude.iter().any(|(class, title)| {
                class.is_match(&client.class) && (title.is_match(&client.title))
            })
        })
        .collect::<Vec<Client>>()
}

fn get_active_client() -> String {
    Client::get_active()
        .unwrap_or(None)
        .map(|x| x.address)
        .unwrap_or(Address::new("0"))
        .to_string()
}
