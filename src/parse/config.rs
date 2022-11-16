use std::fs::read_to_string;

use relative_path::RelativePathBuf;
use serde::Deserialize as Deserialise;
use ron::from_str;
use semver::Version;

use crate::notes::{
    push_error,
    push_warn
};


#[derive(Deserialise)]
pub struct Config {
    project : Project
}

#[derive(Deserialise)]
pub struct Project {
    name    : String,
    #[serde(deserialize_with = "deserialise_version")]
    version : Option<Version>
}

fn deserialise_version<'l, D>(d : D) -> Result<Option<Version>, D::Error>
    where D : serde::Deserializer<'l>
{
    return Ok(Version::parse(<&str>::deserialize(d)?).ok());
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
    push_warn!(InternalWarning, Always, {
        None => {"Todo : Check Project Config"}
    });
}
