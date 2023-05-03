use chrono::{NaiveDate,DateTime, FixedOffset};
use serde::{de};

use std::fs::{File};
use std::io::Write;

use crate::response::SObjectRAWDescribeResponse;



pub fn substring_before(body: &str, separator: &str) -> String {
    match body.find(separator) {
        Some(i) => body.get(..i).unwrap().to_string(),
        None => body.to_string(),
    }
}

pub fn write_to_file(file_name : &str, contents : SObjectRAWDescribeResponse) {
    let mut file = File::create(file_name).expect("Could not create file");
    file.write_all(format!("{:#?}", contents).as_bytes()).unwrap();
}


// DESERIALIZERS FOR OBJECT BUILDER

pub fn deserialize_date<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s = de::Deserialize::deserialize(deserializer);
    match s {
        Ok(s) => {
            let date = NaiveDate::parse_from_str(s, "%Y-%m-%d");    
            match date {
                Ok(date) => Ok(Some(date)),
                    _ => Ok(None)
            }
        }
        Err(_) => Ok(None)
    }
}

pub fn deserialize_datetime<'de, D>(deserializer: D) -> Result<Option<DateTime<FixedOffset>>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s = de::Deserialize::deserialize(deserializer);
    match s {
        Ok(s) => {
            let datetime =   DateTime::parse_from_str(s,"%Y-%m-%dT%H:%M:%S%.3f%z");
            match datetime {
                Ok(datetime) => Ok(Some(datetime)),
                _ => Ok(None)
            }

        }
        Err(_) => Ok(None)
    }
}


pub fn deserialize_bool<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s = de::Deserialize::deserialize(deserializer);
    match s {
        Ok(s) => {
            Ok(s)
        }
        Err(_) => Ok(None)
    }
}


