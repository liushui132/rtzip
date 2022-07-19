use clap::{
    crate_authors, crate_description, crate_name, crate_version, Arg, ArgGroup, Command, ValueHint,ArgMatches,
};

#[derive(Debug,Clone)]       
pub struct Sparse<'a>{    
    pub path :&'a str,
    pub file :&'a str,                   
    pub except_path :Vec<&'a str>,
    pub include_path :Vec<&'a str>,
    pub except_file :Vec<&'a str>,
    pub include_file :Vec<&'a str>,
    pub out:&'a str,    
    pub key:&'a str,
    // out:&'a str,   //输出到文件  todo                      
  
}
pub fn initialize() -> Command<'static>{
    let app = Command::new(crate_name!())
    .version(crate_version!())
    .author(crate_authors!())
    .about(crate_description!())
    .arg(
        Arg::new("path")
            .short('p')
            .long("path")
            .required(false)
            // .default_value("path")
            .value_name("path")
            .help("the path you will compression ")
    )
    .arg(
        Arg::new("paths")
            .short('s')
            .long("paths")
            .required(false)
            // .default_value("path")
            .value_name("paths")
            .help("the paths you will compression ")
    )
    .arg(
        Arg::new("file")
            .short('f')
            .long("file")
            .required(false)
            // .default_value("path")
            .value_name("file")
            .help("the file you will compression ")
    )
    .arg(
        Arg::new("except_path")
            .short('e')
            .long("except_path")
            .required(false)
            .value_name("except_path")
            .help("the file type you don't want to compression")
    )
    .arg(
        Arg::new("include_file")
            .short('n')
            .long("include_file")
            .required(false)
            .value_name("include_file")
            .help("the file type you  want to compression"),
    )
    .arg(
        Arg::new("except_file")
            .short('x')
            .long("except_file")
            .required(false)
            .value_name("except_file")
            .help("the file type you don't want to compression")
    )
    .arg(
        Arg::new("include_path")
            .short('i')
            .long("include_path")
            .required(false)
            .value_name("include_path")
            .help("the file path you  want to compression"),
    )
    
    .arg(
        Arg::new("out")
            .short('o')
            .long("out")
            .required(false)
            .value_name("out")
            .default_value("res")
            .help("the file name you want output ,default is res.txt"),
    )
    .arg(
        Arg::new("key")
            .short('k')
            .long("key")
            .required(false)
            .value_name("key")
            .default_value("txt")
            .help("the password of the zip,default is null"),
    )
    
    .group(
        ArgGroup::new("kind")
        // .args(&["include_path", "except_path"])
        // .args(&["include_file", "except_file"])
        .args(&["path", "file","paths"]).required(true)
        // .required(false)

    )
    .group(
        ArgGroup::new("kind1")
        .args(&["include_path", "except_path"])
        .args(&["include_file", "except_file"])
        // .args(&["path", "file"]).required(true)
        .required(false)

    )
    ;

    app
}


pub fn cmd() -> ArgMatches {
    let app = initialize().get_matches();
    app
}


fn getVec<'a>(data:&'a str) -> Vec<&'a str> {
    if data == "" {
        Vec::new()
    } else {
        data.rsplit(',').collect()
    }
}

pub fn parsecmd<'a>(cmd:&'a ArgMatches) -> Sparse<'a> {

    let path = match cmd.value_of("path") {
        None =>"",
        Some(t) => t,
    };

    let file = match cmd.value_of("file") {
        None =>"",
        Some(t) => t,
    };

    let include = match cmd.value_of("include_path") {
        None =>"",
        Some(t) => t,
    };
    let except = match cmd.value_of("except_path") {
        None =>"",
        Some(t) => t,
    };
    let include_file = match cmd.value_of("include_file") {
        None =>"",
        Some(t) => t,
    };
    let out = match cmd.value_of("out") {
        None =>"res",
        Some(t) => t,
    };
    let except_file = match cmd.value_of("except_file") {
        None =>"",
        Some(t) => t,
    };
    let key = match cmd.value_of("key") {
        None =>"",
        Some(t) => t,
    };

    let include_path_Vec: Vec<&str> = getVec(&include);
    let except_path_Vec: Vec<&str> = getVec(&except);
    let include_file_Vec: Vec<&str> = getVec(&include_file);
    let except_file_Vec: Vec<&str> = getVec(&except_file);

    let temp = Sparse{path:path,file:file,except_path:except_path_Vec,include_path:include_path_Vec,except_file:except_file_Vec,include_file:include_file_Vec,out:out,key:key};
    temp
    
}