fn main() {
    // 解压文件
    extract(Path::new("plugins.zip"), Path::new("server/plugins"));
}

use std::fs;
use std::io::{Read, Write};
use std::path::Path;
// 提取压缩文件中的内容到指定的目标目录
fn extract(zip_file_path: &Path, to_target_path: &Path) {
    // 打开压缩文件
    let zipfile = std::fs::File::open(&zip_file_path).unwrap();
    // 创建一个新的 ZipArchive 对象，用于读取压缩文件
    let mut zip = zip::ZipArchive::new(zipfile).unwrap();

    // 如果目标目录不存在，则创建目标目录
    if !to_target_path.exists() {
        fs::create_dir_all(to_target_path)
            .map_err(|e| {
                println!("{}", e);
            })
            .unwrap();
    }
    // 遍历压缩文件中的每个条目（文件或文件夹）
    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();
        // 如果是文件夹，则创建对应的目录
        if file.is_dir() {
            let target = to_target_path.join(Path::new(&file.name().replace("\\", "")));
            fs::create_dir_all(target).unwrap();
        } else {
            // 如果是文件，则获取文件路径并创建文件
            let file_path = to_target_path.join(Path::new(file.name()));
            if file_path.exists() {
                fs::remove_file(file_path.clone()).unwrap();
            };
            let mut target_file = fs::File::create(file_path.clone()).unwrap();

            let mut buffer = Vec::new();
            // read the whole file
            file.read_to_end(&mut buffer).unwrap();
            target_file.write_all(&buffer).unwrap();
        }
    }
}
