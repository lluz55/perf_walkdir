#![allow(unused)]

use std::{env, fs};

// use walkdir::WalkDir;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum FileType {
  File,
  Dir,
}

#[derive(Debug, Deserialize, Serialize)]
struct FilePtr {
  name: String,
  ty: FileType,
  size: u64,
}

impl FilePtr {
  fn new(name: String) -> Self {
    Self {
      name,
      ty: FileType::Dir,
      size: 0,
    }
  }
}


#[derive(Debug, Deserialize, Serialize)]
struct ReturnedData<T> {
  success: bool,
  data: T,
}

use jwalk::{WalkDir};

// fn read_async(path: String) {
//   use async_walkdir::{Filtering, WalkDir};
//   use futures_lite::future::block_on;
//   use futures_lite::stream::StreamExt;

//   block_on(async {
//       let mut entries = WalkDir::new(path).filter(|entry| async move {
//           if let Some(true) = entry
//               .path()
//               .file_name()
//               .map(|f| f.to_string_lossy().starts_with('.'))
//           {
//               return Filtering::IgnoreDir;
//           }
//           // if entry.file_type().await.unwrap().is_dir() {
//           //   Filtering::IgnoreDir
//           // } else {
//             Filtering::Continue
//           // }
//       });

//       loop {
//           match entries.next().await {
//               Some(Ok(entry)) => println!("file: {}", entry.path().display()),
//               Some(Err(e)) => {
//                   eprintln!("error: {}", e);
//                   break;
//               }
//               None => break,
//           }
//       }
//   });
// }

fn use_jwalk(path: String) {
  for entry in WalkDir::new(path).max_depth(1) {
    let e = entry.unwrap();

    e.file_type().is_dir();

    let file = FilePtr {
      name: e.path().to_str().unwrap().to_owned(),
      ty: if e.file_type().is_dir() { FileType::Dir } else { FileType::File },
      size: if e.file_type().is_dir() { 0 } else { e.metadata().unwrap().len() },
    };
     println!(r#"{{ "name": {}, "size": {}, "type": {:?}}}"#, file.name, file.size, file.ty);

  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  
  let mut path = String::from("/usr/bin");
  
  if args.len() > 1 {
      path = args[1].clone()
    }
    
  use_jwalk(path);
  // for entry in WalkDir::new(path).max_depth(1).into_iter() {      
  //   let e = entry.unwrap();
  //   let metadata = match fs::metadata(e.path().as_os_str()) {
  //     Ok(meta) => meta,
  //     Err(_) => continue
  //   };
    
  //   let file = FilePtr {
  //       name: e.path().to_str().unwrap().to_owned(),
  //       ty: if metadata.is_dir() { FileType::Dir } else { FileType::File },
  //       size: if metadata.is_file() { metadata.len() } else { 0 },
  //   };

    
  //   println!(r#"{{ "name": {}, "size": {}, "type": {:?}}}"#, file.name, file.size, file.ty);
  // }
}
