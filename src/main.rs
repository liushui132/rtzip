
use anyhow::Result;
// use zip::write::FileOptions;
pub mod cmd;
pub mod rzip;
use std::io::Error;
use cmd::parse::{cmd,parsecmd,Sparse};
use rzip::rzip::parsezip;
use std::fs::{read_dir,metadata};



fn main() ->Result<()>{
    let cmd = cmd();
    let data = parsecmd(&cmd);
    println!("{:?}",data);
    parsezip(&data);
    Ok(())
}
