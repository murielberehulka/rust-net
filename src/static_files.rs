use std::collections::HashMap;
use std::fs;
use super::util::get_extension;

pub enum FileType {
    Html, Text, Css, Js, Pdf, Json, Zip, Gif, Jpeg, Png, Svg, Xml, Mpeg, Mp4, Ico, Unknown
}
impl FileType {
    pub fn get(v: &[u8]) -> Self {
        let ext = get_extension(v);
        match ext {
            [b'h',b't',b'm',b'l'] => Self::Html,
            [b't',b'x',b't'] => Self::Text,
            [b'c',b's',b's'] => Self::Css,
            [b'j',b's'] => Self::Js,
            [b'p',b'd',b'f'] => Self::Pdf,
            [b'j',b's',b'o',b'n'] => Self::Json,
            [b'z',b'i',b'p'] => Self::Zip,
            [b'g',b'i',b'f'] => Self::Gif,
            [b'j',b'p',b'g'] => Self::Jpeg,
            [b'j',b'p',b'e',b'g'] => Self::Jpeg,
            [b'p',b'n',b'g'] => Self::Png,
            [b's',b'v',b'g'] => Self::Svg,
            [b'x',b'm',b'l'] => Self::Xml,
            [b'm',b'p',b'e',b'g'] => Self::Mpeg,
            [b'm',b'p',b'4'] => Self::Mp4,
            [b'i',b'c',b'o'] => Self::Ico,
            _ => Self::Unknown
        }
    }
    pub fn to_bytes(&self) -> &'static [u8] {
        match self {
            FileType::Html => "text/html".as_bytes(),
            FileType::Text => "text/plain".as_bytes(),
            FileType::Css => "text/css".as_bytes(),
            FileType::Js => "application/javascript".as_bytes(),
            FileType::Pdf => "application/pdf".as_bytes(),
            FileType::Json => "application/json".as_bytes(),
            FileType::Zip => "application/zip".as_bytes(),
            FileType::Gif => "image/gif".as_bytes(),
            FileType::Jpeg => "image/jpeg".as_bytes(),
            FileType::Png => "image/png".as_bytes(),
            FileType::Svg => "image/svg+xml".as_bytes(),
            FileType::Xml => "text/xml".as_bytes(),
            FileType::Mpeg => "video/mpeg".as_bytes(),
            FileType::Mp4 => "video/mp4".as_bytes(),
            FileType::Ico => "image/x-icon".as_bytes(),
            FileType::Unknown => "unknown".as_bytes()
        }
    }
}

pub type File = (&'static [u8], Vec<u8>);
pub type Files = HashMap<Vec<u8>, File>;

pub struct StaticFiles {
    pub root_path: &'static str,
    pub cache: Option<Files>
}

impl StaticFiles {
    pub fn new(root_path: &'static str, use_cache: bool) -> StaticFiles {
        if use_cache {
            let mut cache: Files = HashMap::new();
            let root_folder = match fs::read_dir(root_path) {
                Ok(v) => v,
                Err(e) => panic!("Static files folder not found, path: \"{}\", error: {}", root_path, e)
            };
            for path in root_folder {
                for_eatch_path(path.unwrap().path(), root_path, &mut cache);
            }
            StaticFiles {
                root_path,
                cache: Some(cache)
            }
        }else {
            StaticFiles {
                root_path,
                cache: None
            }
        }
    }
}

fn for_eatch_path(path: std::path::PathBuf, root_path: &'static str, cache: &mut Files) {
    if path.is_dir() {
        for path_inside in fs::read_dir(&path).unwrap() {
            for_eatch_path(path_inside.unwrap().path(), root_path, cache);
        }
    }else {
        let file_name = path.strip_prefix(root_path).unwrap().to_str().unwrap().replace('\\', "/");
        if let Some(file) = read_file(root_path, file_name.as_bytes()) {
            cache.insert(Vec::from(file_name.as_bytes()), file);
        }
    }
}

pub fn read_file(root_path: &'static str, path: &[u8]) -> Option<File> {
    let file_type = FileType::get(path).to_bytes();
    let path_str = String::from_utf8_lossy(path).to_string().replace("%20", " ");
    let full_path = format!("{}/{}", root_path, path_str);
    match fs::read(full_path) {
        Ok(content) => 
            Some((file_type, content)),
        Err(_) => None
    }
}