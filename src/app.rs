// Copyright (c) 2021-2021 The Pastel Core developers
// Distributed under the MIT software license, see the accompanying
// file COPYING or http://www.opensource.org/licenses/mit-license.php.

use clap::{Arg, App, ArgMatches};
use config::{ConfigError, Config, File};
use std::env;
use dirs;

const NIX_PASTELD_PATH: &str = ".pastel";
const MAC_PASTELD_PATH: &str = "Library/Application Support/Pastel";
const WIN_PASTELD_PATH: &str = "AppData\\Roaming\\Pastel";
const DEFAULT_CONFIG_FILE: &str = "rqservice";

pub struct ServiceSettings {
    pub grpc_service: String,
    pub pastel_path: String,
    pub config_path: String
}

impl ServiceSettings {

    pub fn new() -> Result<Self, ConfigError> {

        let pastel_path;
        let config_path;

        match dirs::home_dir() {
            Some(path) => {
                if env::consts::OS == "linux" {
                    pastel_path = format!("{}/{}", path.display(), NIX_PASTELD_PATH);
                    config_path = format!("{}/{}", pastel_path, DEFAULT_CONFIG_FILE);
                } else if env::consts::OS == "macos" {
                    pastel_path = format!("{}/{}", path.display(), MAC_PASTELD_PATH);
                    config_path = format!("{}/{}", pastel_path, DEFAULT_CONFIG_FILE);
                } else if env::consts::OS == "windows" {
                    pastel_path = format!("{}\\{}", path.display(), WIN_PASTELD_PATH);
                    config_path = format!("{}\\{}", pastel_path, DEFAULT_CONFIG_FILE);
                } else {
                    panic!("Unsupported system!");
                }
            },
            None => panic!("Unsupported system!")
        }

        let cmd_args = ServiceSettings::cmd_args_new(&config_path);
        let cfg = ServiceSettings::init_cfg(&config_path, &cmd_args);

        let grpc_service = ServiceSettings::find_setting(cmd_args, cfg, "grpc-service", "".to_string(), true);

        Ok(ServiceSettings{grpc_service, pastel_path, config_path})
    }

    fn cmd_args_new(config_path: &String) -> ArgMatches<'static> {
        App::new("rqservice")
            .version("v0.1.0")
            .author("Pastel Network <pastel.network>")
            .about("RaptorQ Service")
            .arg(Arg::with_name("config")
                .short("c")
                .long("config-file")
                .value_name("FILE")
                .help(format!("Set path to the config file. (default: {})", config_path).as_str())
                .takes_value(true))
            .arg(Arg::with_name("grpc-service")
                .short("s")
                .long("grpc-service")
                .value_name("IP:PORT")
                .help("Set IP address and PORT for gRPC server to listen on. (default: 127.0.0.1:50051)")
                .takes_value(true))
            .get_matches()
    }

    fn init_cfg(config_path: &String, cmd_args: &ArgMatches) -> config::Config {
        let config_file = cmd_args.value_of("config").unwrap_or(&config_path);

        let mut cfg = Config::default();
        if let Err(err) = cfg.merge(File::with_name(&config_file)) {
            println!("Cannot read config file {} - {}", config_file, err);
        }

        cfg
    }

    fn find_setting( args: ArgMatches, cfg: Config, name: &str, default: String, must: bool ) -> String {
        let param: String;
        match args.value_of(&name) {
            Some(v) => param = v.to_string(),
            None => {
                match cfg.get::<String>(&name) {
                    Ok(v) => param = v,
                    Err(err) => {
                        if must {
                            panic!("Parameter {} not found - {}", &name, err)
                        } else {
                            param = default;
                        }
                    }
                }
            }
        }
        param
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn no_file_no_cmd() {
        let config_path= "".to_string();

        let cmd_args = ServiceSettings::cmd_args_new(&config_path);
        let cfg = ServiceSettings::init_cfg(&config_path, &cmd_args);

        ServiceSettings::find_setting(cmd_args, cfg, "grpc-service", "".to_string(), true);
    }
    #[test]
    fn file_but_no_cmd() {
        let config_path= "examples/rqconfig.toml".to_string();

        let cmd_args = ServiceSettings::cmd_args_new(&config_path);
        let cfg = ServiceSettings::init_cfg(&config_path, &cmd_args);

        let grpc_service = ServiceSettings::find_setting(cmd_args, cfg, "grpc-service", "".to_string(), true);
        assert_eq!(grpc_service, "127.0.0.1:50051");
    }
    #[test]
    fn no_file_but_cmd() {
        let config_path= "".to_string();

        let arg_vec = vec!["rqservice", "--grpc-service", "127.0.0.1:50051"];
        let cmd_args = App::new("rqservice")
            .version("v0.1.0")
            .author("Pastel Network <pastel.network>")
            .about("RaptorQ Service")
            .arg(Arg::with_name("config")
                .short("c")
                .long("config-file")
                .value_name("FILE")
                .help(format!("Set path to the config file. (default: {})", config_path).as_str())
                .takes_value(true))
            .arg(Arg::with_name("grpc-service")
                .short("s")
                .long("grpc-service")
                .value_name("IP:PORT")
                .help("Set IP address and PORT for gRPC server to listen on. (default: 127.0.0.1:50051)")
                .takes_value(true))
            .get_matches_from(arg_vec);

        let cfg = ServiceSettings::init_cfg(&config_path, &cmd_args);

        let grpc_service = ServiceSettings::find_setting(cmd_args, cfg, "grpc-service", "".to_string(), true);
        assert_eq!(grpc_service, "127.0.0.1:50051");
    }
    #[test]
    fn file_and_cmd() {
        let config_path= "examples/rqconfig.toml".to_string();

        let arg_vec = vec!["rqservice", "--grpc-service", "127.0.0.1:50052"];
        let cmd_args = App::new("rqservice")
            .version("v0.1.0")
            .author("Pastel Network <pastel.network>")
            .about("RaptorQ Service")
            .arg(Arg::with_name("config")
                .short("c")
                .long("config-file")
                .value_name("FILE")
                .help(format!("Set path to the config file. (default: {})", config_path).as_str())
                .takes_value(true))
            .arg(Arg::with_name("grpc-service")
                .short("s")
                .long("grpc-service")
                .value_name("IP:PORT")
                .help("Set IP address and PORT for gRPC server to listen on. (default: 127.0.0.1:50051)")
                .takes_value(true))
            .get_matches_from(arg_vec);

        let cfg = ServiceSettings::init_cfg(&config_path, &cmd_args);

        let grpc_service = ServiceSettings::find_setting(cmd_args, cfg, "grpc-service", "".to_string(), true);
        assert_eq!(grpc_service, "127.0.0.1:50052");
        assert_ne!(grpc_service, "127.0.0.1:50051");
    }
}