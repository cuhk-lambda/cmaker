pub use path::*;

mod path {
    use std::borrow::Cow;
    use std::path::Path;

    use path_dedot::ParseDot;

    use crate::global::*;

    pub fn work_dir(abs_path: &str) -> &str {
        return &abs_path[0..abs_path.len() - 9];
    }

    pub fn path_without_dot<'a>(work_dir: &str, rev_path: &str) -> String {
        let w = Path::new(work_dir);
        let r = Path::new(rev_path);
        w.join(r).parse_dot().unwrap().to_str().unwrap().to_string()
    }


    pub fn get_last(content: &str) -> String {
        let bytes = content.as_bytes();
        let mut i = content.len();
        while bytes[i - 1] != '/' as u8 {
            i -= 1;
        }
        String::from(&content[i..content.len()])
    }

    pub fn get_abs(content: &Cow<String>, sep: &str, ending: &str, work_dir: &str) -> String {
        let mut buffer = Vec::new();
        let mut state = 0;
        let bytes = content.as_bytes();
        let length = bytes.len();
        if !content.starts_with("cd") {
            state = 1;
            buffer.resize(work_dir.len(), 0);
            buffer.copy_from_slice(work_dir.as_bytes());
            buffer.push('/' as u8);
        }
        for i in 3..length {
            if length - i >= 3 && &content[i..i + 3] == " &&" {
                buffer.push('/' as u8);
                state = 1;
            }
            if i > sep.len() && &content[i - sep.len()..i] == sep {
                state = 2;
            }
            if state == 2 && &content[i - ending.len()..i] == ending {
                state = 3;
            }
            if state == 0 || state == 2 {
                buffer.push(bytes[i]);
            }
        }
        String::from_utf8(buffer).unwrap()
    }
}