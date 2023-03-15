use clap::Parser;
use serde_json::Value;
use shimlib::{defs::*, *};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use uuid::Uuid;
use windows::{core::PCWSTR, Win32::Foundation::*};

// Setup Commandline args
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// JSON config file
    config: String,

    /// Debug printing
    #[arg(short, long)]
    debug: bool,

    /// Also install built SDB
    #[arg(short, long)]
    install: bool,
}

fn handle_list(db: HANDLE, list: &Vec<Value>, debug: bool) -> Result<(), Box<dyn Error>> {
    for i in list {
        let tag_name = i["tag"]
            .as_str()
            .unwrap_or_else(|| panic!("tag should be string but got: {}", i["tag"]));
        let tag: TAG = get_tag(tag_name).unwrap_or_else(|e| panic!("{e}"));
        let data_type = i["type"]
            .as_str()
            .unwrap_or_else(|| panic!("type should be string but got: {}", i["type"]));
        let ival = &i["value"];
        if debug {
            println!("Tag Name: {}  Tag: {} | Type: {}", tag_name, tag, data_type);
        }

        match data_type {
            "NULL" => unsafe {
                SdbWriteNULLTag(db, tag);
            },
            "LIST" => {
                let tag_list: TAGID;
                unsafe {
                    tag_list = SdbBeginWriteListTag(db, tag);
                }
                let value = ival
                    .as_array()
                    .unwrap_or_else(|| panic!("LIST should be a list but got: {}", ival));
                handle_list(db, value, debug)?;
                unsafe {
                    SdbEndWriteListTag(db, tag_list);
                }
            }
            "STRING" => {
                let value = ival
                    .as_str()
                    .unwrap_or_else(|| panic!("STRING should be a string but got: {}", ival));
                let value = get_pwstr(value);
                unsafe {
                    SdbWriteStringTag(db, tag, value);
                }
            }
            "WORD" => {
                let value = ival
                    .as_u64()
                    .unwrap_or_else(|| panic!("WORD should be a number but got: {}", ival));
                let value: u16 = value
                    .try_into()
                    .unwrap_or_else(|_| panic!("{} is to big for WORD", value));
                unsafe {
                    SdbWriteWORDTag(db, tag, value);
                }
            }
            "DWORD" => {
                let value = ival
                    .as_u64()
                    .unwrap_or_else(|| panic!("WORD should be a number but got: {}", ival));
                let value: u32 = value
                    .try_into()
                    .unwrap_or_else(|_| panic!("{} is to big for DWORD", value));
                unsafe {
                    SdbWriteDWORDTag(db, tag, value);
                }
            }
            "QWORD" => {
                let value = ival
                    .as_u64()
                    .unwrap_or_else(|| panic!("WORD should be a number but got: {}", ival));
                unsafe {
                    SdbWriteQWORDTag(db, tag, value);
                }
            }
            "GUID" => {
                let value = ival
                    .as_str()
                    .unwrap_or_else(|| panic!("GUID should be a string in format {{aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa}} but got: {}", ival));
                let value = Uuid::parse_str(value)
                    .unwrap_or_else(|_| panic!("GUID should be in format {{aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa}} but got: {}", ival));
                let mut value = *value.as_bytes();
                unsafe {
                    SdbWriteBinaryTag(db, tag, value.as_mut_ptr(), value.len().try_into().unwrap());
                }
            }
            "BINARY" => {
                let value = ival.as_str().unwrap_or_else(|| {
                    panic!("BINARY should be a Base64 string but got: {}", ival)
                });
                let mut value = base64::decode(value).unwrap_or_else(|_| {
                    panic!("BINARY should be a Base64 string but got: {}", ival)
                });
                unsafe {
                    SdbWriteBinaryTag(db, tag, value.as_mut_ptr(), value.len().try_into().unwrap());
                }
            }
            _ => {
                panic!("ERROR: data_type {}", data_type)
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // Open JSON config file
    let file = File::open(args.config).expect("Can't open config file");
    let data: Value = serde_json::from_reader(BufReader::new(file))?;

    // Create database
    let db_name: PCWSTR = get_pwstr(data["name"].as_str().expect("Set 'name' for DataBase name"));
    let db: HANDLE;
    unsafe {
        db = SdbCreateDatabase(db_name, PATH_TYPE::DOS_PATH);
    }

    // Create database from JSON, inital is always a list
    let data = data["data"]
        .as_array()
        .expect("data should always start as a list");
    handle_list(db, data, args.debug)?;

    // Finalise database file
    unsafe {
        SdbCloseDatabaseWrite(db);
    }

    Ok(())
}
