use std::fs;
use std::io;
use std::path;

fn main() -> io::Result<()> {
    let mut paths = vec![];
    read_recursively(".", &mut paths)?;
    for p in paths {
        let src = p.canonicalize()?;
        let name = src
            .file_name()
            .expect(&format!("file {:?} has no name", src));
        let path = src
            .parent()
            .expect(&format!("file {:?} has no parent", src));

        let lower_dst = path.join(
            name.to_str()
                .unwrap()
                .chars()
                .map(|c| c.to_ascii_lowercase())
                .collect::<String>(),
        );
        let upper_dst = path.join(
            name.to_str()
                .unwrap()
                .chars()
                .map(|c| c.to_ascii_uppercase())
                .collect::<String>(),
        );

        if !lower_dst.exists() {
            println!("Linking {:?} → {:?}", &src, &lower_dst);
            std::os::unix::fs::symlink(&src, lower_dst)?;
        }

        if !upper_dst.exists() {
            println!("Linking {:?} → {:?}", &src, &upper_dst);
            std::os::unix::fs::symlink(&src, upper_dst)?;
        }
    }
    Ok(())
}

fn read_recursively<P: AsRef<path::Path>>(path: P, out: &mut Vec<path::PathBuf>) -> io::Result<()> {
    for file in fs::read_dir(path)? {
        let file = file?;
        let path = file.path();
        if file.file_type()?.is_dir() {
            read_recursively(&path, out)?;
        }
        out.push(path);
    }
    Ok(())
}
