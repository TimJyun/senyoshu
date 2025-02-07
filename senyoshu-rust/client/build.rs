use std::fs::{DirBuilder, File};
use std::io::Write;

use chrono::Utc;

// use std::fs::{File, read_dir};
// use std::io::{Read, Write};
// use std::path::Path;
//
// use blake2::{Blake2b512, Digest};
//
fn main() {
    let _ = DirBuilder::new().recursive(true).create("../target/dist/");

    let mut build_time = File::create("../target/dist/version.txt").unwrap();
    let now = Utc::now();

    build_time
        .write_all(now.format("%Y%m%d").to_string().as_bytes())
        .unwrap();
}
//
// fn main() {
//     let mut files = get_files("../target/dist");
//     files.sort_by_cached_key(|items| items.0.to_string());
//     let mut inventory = File::create("../target/dist/inventory.json").unwrap();
//     let files = files.into_iter().map(|(path, hash)| {
//         let path = path.chars()
//             .enumerate()
//             .filter(|(idx, c)| {
//                 *idx > path.find("dist").unwrap() + 4
//             })
//             .map(|(_, c)| c)
//             .collect();
//         (path, hash)
//     })
//         .filter(|(path, _)| { path != "inventory.json" })
//         .collect::<Vec<(String, String)>>();
//
//     inventory.write_all(serde_json::to_string(&files).unwrap().as_bytes()).unwrap();
//
//
//     let mut version_hash = File::create("../target/dist/version_hash.txt").unwrap();
//     let mut hasher = Blake2b512::new();
//     for (f, h) in files.into_iter() {
//         hasher.update(f);
//         hasher.update(h);
//     }
//     let hash_bytes = hasher.finalize();
//     let hash = to_hex(hash_bytes.as_slice());
//     version_hash.write_all(hash.as_bytes()).unwrap();
// }
//
//
// fn get_files<P: AsRef<Path>>(path: P) -> Vec<(String, String)> {
//     let mut rv = Vec::new();
//     let dir = read_dir(path).unwrap();
//     for item in dir.into_iter() {
//         let item = item.unwrap();
//         let t = item.file_type().unwrap();
//         if t.is_dir() {
//             let mut tmp = get_files(item.path());
//             rv.append(&mut tmp);
//         } else {
//             let mut hasher = Blake2b512::new();
//             let mut f = File::open(item.path()).unwrap();
//             let mut buff = Vec::with_capacity(8192);
//             f.read_to_end(&mut buff).unwrap();
//             hasher.update(buff.as_slice());
//             let hash_output = hasher.finalize();
//             let hash_bytes = hash_output.as_slice();
//             let mut hash = to_hex(hash_bytes);
//             rv.push((item.path().to_str().unwrap().to_string(), hash))
//         }
//     }
//
//
//     rv
// }
//
// fn to_hex(bytes: &[u8]) -> String {
//     let mut rv = String::with_capacity(2 * bytes.len());
//     for byte in bytes {
//         for c in format!("{:02X}", byte).chars() {
//             rv.push(c);
//         }
//     }
//     rv
// }
