pub use path::*;

mod path {
    use std::borrow::Cow;

    pub fn output_canonicalize(x: &str) -> String {
        let mut state = 0;
        let mut res = Vec::new();
        for i in x.split_ascii_whitespace() {
            let mut flag = false;
            if i == "cd" {
                state = 1;
            } else if state == 1 {
                std::env::set_current_dir(i).expect("unable to switch dir");
                state = 0;
            } else if i == "-o" {
                state = 2;
            } else if state == 2 {
                flag = true;
                state = 0;
            }
            if flag {
                res.push(std::fs::canonicalize(i).unwrap().to_str().unwrap().to_string())
            } else {
                res.push(i.to_string())
            }
        }
        res.join(" ")
    }
    pub fn path_without_dot<'a>(rev_path: &str) -> String {
        match std::fs::canonicalize(rev_path) {
            Ok(e) => e.to_str().unwrap().to_string(),
            Err(e) => {
                println!("current dir: {}", std::env::current_dir().unwrap().to_str().unwrap());
                println!("failed path: {}, {}", rev_path, e);
                std::process::exit(0)
            }
        }
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
                let path = String::from_utf8(buffer.clone()).unwrap();
                std::env::set_current_dir(path.as_str()).expect(format!("unable to change dir to {}", path).as_str());
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