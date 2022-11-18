use std::fs::read_to_string;

use relative_path::RelativePathBuf;
use serde::Deserialize as Deserialise;
use ron::from_str;
use semver::Version;

use crate::notes::{
    push_error,
    push_warn
};


const ALLOWED_PROJECT_NAME_CHARS : &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_";


#[derive(Deserialise)]
pub struct Config {
    pub project : Project
}

#[derive(Deserialise)]
pub struct Project {
    pub name    : String,
    #[serde(deserialize_with = "deserialise_version")]
    pub version : Result<Version, (String, semver::Error)>
}

fn deserialise_version<'l, D>(d : D) -> Result<Result<Version, (String, semver::Error)>, D::Error>
    where D : serde::Deserializer<'l>
{
    let version = <&str>::deserialize(d)?;
    return Ok(Version::parse(version).map_err(|e| (version.to_owned(), e)));
}


pub(crate) fn read(path : &RelativePathBuf) -> Option<Config> {
    return match (read_to_string(&path.join("config").with_extension("vsv.ron").as_str())) {
        Ok(config) => match (from_str(&config)) {
            Ok(config) => {
                check(&config);
                Some(config)
            },
            Err(error) => {
                push_error!(ModuleNotFound, Always, {
                    None => {"{}", error},
                    None => {"`config.vsv.ron` failed to load."}
                });
                None
            }
        },
        Err(error) => {
            push_error!(ModuleNotFound, Always, {
                None => {"{}", error},
                None => {"`config.vsv.ron` failed to load."}
            });
            None
        }
    };
}

fn check(config : &Config) {
    // Project
    {
        // Name
        {
            let mut invalid = Vec::new();
            for ch in config.project.name.chars() {
                if (! ALLOWED_PROJECT_NAME_CHARS.contains(ch)) {
                    invalid.push(ch);
                }
            }
            if (invalid.len() > 0) {
                push_error!(ConfigProjectInvalidName, Always, {
                    None => {"Project name contains invalid character{}: {}",
                        if (invalid.len() != 1) {"s"} else {""},
                        if (invalid.len() == 1) {
                            format!("`{}`", invalid[0])
                        } else {
                            let last = invalid.remove(invalid.len() - 1);
                            format!("{}, or {}",
                                invalid.iter()
                                    .map(|ch| format!("`{}`", ch))
                                    .collect::<Vec<_>>().join(", "),
                                format!("`{}`", last)
                            )
                        }
                    }
                });
            }
        }
        // Version
        if let Err((text, error)) = &config.project.version {
            push_error!(ConfigProjectInvalidName, Always, {
                None => {"Project version `{}` is invalid", text},
                None => {"{}", error}
            });
            return;
        }
    }
}
