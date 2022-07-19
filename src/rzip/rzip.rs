
use std::fs;
use std::fs::File;
use std::iter::Chain;
use std::slice::Iter;

use std::io::{copy, Read, Seek, Write};
use std::path::Path;
use std::str;

use walkdir::{DirEntry, WalkDir};
use zip::write::FileOptions;
use crate::cmd::parse::Sparse;



// 获取目录，然后根据规律进行过滤
// 这样写的话 ，就限制了在
pub fn parsezip<'a>(parsedata:&'a Sparse ) -> anyhow::Result<()> {

    let mut pathVec = Path::new(parsedata.path);
    let mut fileVec = Path::new(parsedata.file);
    if parsedata.file != "" {
        compress_file(fileVec,Path::new(parsedata.out));  
    } else {
        // compress_dir(pathVec,Path::new(parsedata.out),&parsedata.except_file);  
        compress_dir(pathVec,Path::new(parsedata.out),&parsedata.except_file);   
    }
    

    // let mut pathiter =std::iter::empty::<DirEntry<>>();
    // let mut  tempdata:Vec<&Path> = Vec::new();


    println!("comparess okk,path is {:?}",parsedata.out);
    Ok(())
    }



fn get_filter_type<'a>(entry:&DirEntry,filter_type:&Vec<&'a str>) ->bool{

    for value in filter_type.into_iter() {
        if (entry.file_name().to_str().map(|s| !s.ends_with(value)).unwrap() == false) {
            return false;
        }        
    }    
    return true
    
    // entry
    //     .file_name()
    //     .to_str()
    //     .map(|s| !s.ends_with(".d")).unwrap()
}



fn compress_dir<'a>(src_dir: &Path, target: &Path,filter_type:&Vec<&'a str>) {
    let zipfile = std::fs::File::create(target).unwrap();
    println!("compress_dir");
    let dir = WalkDir::new(src_dir);
    zip_dir(&mut dir.into_iter().filter_entry(|entry|get_filter_type(entry,filter_type) ).filter_map(|e| e.ok()), src_dir.to_str().unwrap(), zipfile);
}
/// 压缩文件
fn compress_file(src_dir: &Path, target: &Path) {
    // let zipfile = std::fs::File::create(target).unwrap();
    // let dir = WalkDir::new(src_dir);
    // println!("compress_file");
    // // let prefix = src_dir.parent().map_or_else(||"/",|p|p.to_str().unwrap());

    // zip_dir(&mut dir.into_iter().filter_map(|e| e.ok()), src_dir.to_str().unwrap(), zipfile);
    let zipfile = std::fs::File::create(target).unwrap();
    println!("compress_file");
    let dir = WalkDir::new(src_dir);
    zip_dir(&mut dir.into_iter().filter_map(|e| e.ok()), src_dir.to_str().unwrap(), zipfile);


}


fn zip_dir<T>(it: &mut dyn Iterator<Item=DirEntry>, prefix: &str, writer: T) -> zip::result::ZipResult<()>
    where T: Write + Seek {
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Bzip2)//直接用了bzip2压缩方式，其它参看枚举
        .unix_permissions(0o755);//unix系统权限
 
    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        //zip压缩一个文件时，会把它的全路径当成文件名(在下面的解压函数中打印文件名可知)
        //这里是去掉目录前缀
        println!("  {:?} ", entry);
        let name = path.strip_prefix(Path::new(prefix)).unwrap();
        // println!("name  {:?} ...", name);
        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            // println!("adding file {:?} as {:?} ...", path, name);
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;
 
            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        } else if name.as_os_str().len() != 0 {//目录
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            // println!("adding dir {:?} as {:?} ...", path, name);
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;
    Result::Ok(())
}
