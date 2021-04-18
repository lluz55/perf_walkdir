use std::{fs, thread, thread::{sleep}, time::Duration};

use lazy_static::lazy_static;
use parking_lot::Mutex;
// use walkdir::WalkDir;
use serde::{Deserialize, Serialize};
use std::ffi::{CString, CStr};
use libc::{c_char, c_int};

use jwalk::WalkDir;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
enum FileType {
  File,
  Dir,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct File {
  name: String,
  ty: FileType,
  size: u64,
}

impl File {
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
  data: T,
}

#[repr(C)]
pub struct NativeResp<T> {
  error: c_int,
  error_msg: *mut c_char,
  data: T
}


#[repr(C)]
pub struct FilePtr {
  name: *mut c_char,
  ty: c_int,
  size: u64,
}

#[repr(C)]
pub struct Buffer {
  data: *mut FilePtr,
  len: i32,
}

// TODO: BEST IMPLMENTATION

#[no_mangle]
pub extern "C" fn get_files_native(c_path: *mut c_char, cb: extern fn(FilePtr)) {
  let path = to_r(c_path);

  thread::spawn(|| {    
    loop {
      println!("sleeping");
      sleep(Duration::from_secs(1));
    }
  });

  for entry in WalkDir::new(path).max_depth(1) {
    let e = entry.unwrap();

    let file = FilePtr {
      name: to_c(e.path().to_str().unwrap()),
      ty: if e.file_type().is_dir() { 0 } else { 1 },
      size: if e.file_type().is_dir() { 0 } else { e.metadata().unwrap().len() },
    };
    
    cb(file);
  }
}












#[no_mangle]
pub extern "C" fn get_files() -> *mut c_char{

  let mut files_list: Vec<File> = vec![];

  for _ in 0..1 {
    for entry in WalkDir::new("/usr/bin").max_depth(1) {      
      let e = entry.unwrap();
      let metadata = fs::metadata(e.path().as_os_str()).unwrap();

      let mut file = File::new(e.path().to_str().unwrap().to_owned());

      if metadata.is_file() {
        file.size = metadata.len();
        file.ty = FileType::File;
      }

      files_list.push(file);           
    }
  }

  let data = ReturnedData::<Vec<File>>{
      data: files_list,
  };

  let result = serde_json::to_string(&data).unwrap();
  CString::new(result).unwrap().into_raw()
}




lazy_static! {
  pub static ref QUEUE: Mutex<DataResult> = Mutex::new(DataResult::None);
  pub static ref STARTED: Mutex<bool> = Mutex::new(false);
}

pub enum DataResult {
  Data(String),
  Exit,
  None,
}

#[no_mangle]
pub extern "C" fn get_files_stream(cb: extern fn(*mut c_char)) {  
  for _ in 0..1 {
    let mut files_list: Vec<File> = vec![];

    for (index, entry) in WalkDir::new("/usr/bin").max_depth(1).into_iter().enumerate() {      
      let e = entry.unwrap();
      let metadata = fs::metadata(e.path().as_os_str()).unwrap();
      let mut file = File::new(e.path().to_str().unwrap().to_owned());

      if metadata.is_file() {
        file.size = metadata.len();
        file.ty = FileType::File;
      }
      files_list.push(file);
      if index % 5 == 0 {
        let data = ReturnedData{
          data: files_list.clone(),
        };
        let c_data = serde_json::to_string(&data).unwrap();
        cb(CString::new(c_data).unwrap().into_raw());
        files_list.clear();
      }
    }

    // let c_files = serde_json::to_string(&files_list).unwrap();
    // let file_ptr = CString::new(c_files).unwrap().into_raw();
    let data = ReturnedData{
      data: files_list,
    };
    let c_data = serde_json::to_string(&data).unwrap();
    cb(CString::new(c_data).unwrap().into_raw());
  }
  cb(CString::new("").unwrap().into_raw());
}

fn to_c(data: &str) -> *mut c_char {
  CString::new(data).unwrap().into_raw()
}

fn to_r(c_data: *mut c_char) -> String {
  unsafe {
    if c_data.is_null() {
      println!("String can't be null");
      return "".to_string()
    }
    CStr::from_ptr(c_data).to_str().unwrap().to_string()  
  }
}


#[no_mangle]
pub extern "C" fn get_async(c_route: *const c_char, c_payload: *const c_char, cb: extern fn(*mut c_char, *mut c_char)) {
  let null_error_route = to_c("error");
  let route = unsafe {
    if c_route.is_null() {
      let resp = CString::new("route can't be null").unwrap().into_raw();
      cb(null_error_route, resp);
      return;
    }
    CStr::from_ptr(c_route).to_str().unwrap()
  };
  
  let payload = unsafe {
    if c_payload.is_null() {
      let resp = CString::new("payload can't be null").unwrap().into_raw();
      cb(null_error_route, resp);
      return;
    }
    CStr::from_ptr(c_payload).to_str().unwrap()
  };

   match route {
    "ping" => {
      cb(to_c("ping"), to_c(payload));
    },
    "get_dir_list" => {

      for entry in WalkDir::new(payload).max_depth(1) {
        let e = entry.unwrap();

        let file = File {
          name: e.path().to_str().unwrap().to_owned(),
          ty: if e.file_type().is_dir() { FileType::Dir } else { FileType::File },
          size: if e.file_type().is_dir() { 0 } else { e.metadata().unwrap().len() },
        };

        let c_data = serde_json::to_string(&file).unwrap();
        cb(to_c("get_dir_list"), to_c(c_data.as_ref()));
      }
    }
    _ => ()
  }
}

// TODO: run program from here:
// TODO: c;cargo build --lib --release ; ../../list_files_in_dir/read_files_from_rust/bin/read_files_from_rust.exe


#[no_mangle]
pub extern "C" fn free_resources(raw_str: *mut c_char) {
  unsafe {
    if raw_str.is_null() { return }
    CString::from_raw(raw_str);
  }
}