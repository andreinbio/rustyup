
use std::fs::{File, read_dir};
use std::io::{Error, prelude::*};

use serde::{Serialize};
use serde::de::DeserializeOwned;

pub trait Config: Sized {
    fn save<>() -> Result<(), Error>;
    fn load<>() -> Result<Self, Error>;
}

impl <T> Config for T 
    where T: Serialize + DeserializeOwned + Sized
    {
        fn save<>() -> Result<(), Error> {
            let mut file = File::create("config.json")?;
            let config = T::load()?;
            let json = serde_json::to_string(&config)?;
            file.write_all(json.as_bytes())?;
            Ok(())
        }

       fn load<>() -> Result<Self, Error> {
            let mut file = File::open("config.json")?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            let config = serde_json::from_str(&contents)?;
            Ok(config)
       }
    }
