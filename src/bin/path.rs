use std::{env, io};
use std::path::{Path, PathBuf};

fn get_abs_path() -> io::Result<PathBuf> {
    let dir = "logs";
    let file_name = "tmp/default.log";
    let mut path = PathBuf::from(file_name);
    if path.is_absolute() {
        return Ok(path);
    }

    path = PathBuf::from(dir);
    if path.is_absolute() {
        return Ok(path.join(file_name));
    }
    

    path = env::current_dir()?;
    println!("cur_dir: {}", path.to_str().unwrap());
    Ok(path.join(format!("{}/{}", dir, file_name)))
}

fn with_file_name() {
    let path = Path::new("/tmp/foo.png");
    assert_eq!(path.with_file_name("bar"), PathBuf::from("/tmp/bar"));
    assert_eq!(path.with_file_name("bar.txt"), PathBuf::from("/tmp/bar.txt"));

    let path = Path::new("/tmp");
    assert_eq!(path.with_file_name("var"), PathBuf::from("/var"));

}

fn join() {
    println!("join path");
    let mut path = PathBuf::from("/tmp");
    path = path.join("");
    println!("{}", path.to_str().unwrap());
    path = path.join("default.log");
    println!("{}", path.to_str().unwrap());
}

fn get_parent() {
    println!("get_parent");
    let path = PathBuf::from("/tmp/foo/bar.txt");
    println!("{} 's parent dir: {}", path.to_str().unwrap(), path.parent().unwrap().to_str().unwrap());
}

fn main() {
    let abs_path = get_abs_path().unwrap();
    println!("{}", abs_path.to_str().unwrap());
    
    with_file_name();
    join();
    get_parent();
}