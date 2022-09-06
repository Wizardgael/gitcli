use std::process::{Command, Output};

#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub enum GitFile{
    FileUntracked(String),
    FileChached(String),
    FileModified(String)
}

impl GitFile {

    pub fn name(&self) -> String {
        match self {
            GitFile::FileChached(name) => name.clone(),
            GitFile::FileModified(name) => name.clone(),
            GitFile::FileUntracked(name) => name.clone()
        }
    }

}

pub struct Git{

}

impl Git{

    fn command(com: String) -> Output{
        let mut command = if cfg!(target_os = "windows"){
            let mut c = Command::new("cmd");
            c.arg("/C");
            c
        }else{
            let mut c = Command::new("sh");
            c.arg("-c");
            c
        };
        command.arg(com);

        command.output().expect("Error during command")
    }


    fn untracked_file() -> Result<Vec<String>, String>{
        let output = Git::command("git ls-files . --exclude-standard --others".into());
        if let Ok(s) = String::from_utf8(output.stdout) {
            Ok(s.trim().split("\n").map(|f| f.to_string()).collect())
        }else {
            return Err("Error reading untracked files".into())
        }
    }

    fn modified_file() -> Result<Vec<String>, String>{
        let output = Git::command("git ls-files . --exclude-standard --modified".into());
        if let Ok(s) = String::from_utf8(output.stdout) {
            Ok(s.trim().split("\n").map(|f| f.to_string()).collect())
        }else {
            return Err("Error reading untracked files".into())
        }
    }

    fn cached_file() -> Result<Vec<String>, String>{
        let output = Git::command("git ls-files . --exclude-standard --cached".into());
        if let Ok(s) = String::from_utf8(output.stdout) {
            Ok(s.trim().split("\n").map(|f| f.to_string()).collect())
        }else {
            return Err("Error reading untracked files".into())
        }
    }



    

    pub fn get_file() -> Result<Vec<GitFile>, String> {
        let mut res: Vec<GitFile> = Vec::new();
        let mut tmp: Vec<String> = Vec::new();
        let untracked = Git::untracked_file()?;
        untracked.iter().for_each(|f| {
            tmp.push(f.clone());
            res.push(GitFile::FileUntracked(f.clone()));
        });
        let modified = Git::modified_file()?;
        modified.iter().for_each(|f|{
            if tmp.contains(f) {
                return;
            }
            tmp.push(f.clone());
            res.push(GitFile::FileModified(f.clone()));
        });
        let cached = Git::cached_file()?;
        cached.iter().for_each(|f|{
            if tmp.contains(f) {
                return;
            }
            tmp.push(f.clone());
            res.push(GitFile::FileChached(f.clone()));
        });
        res.sort_by(|a, b| {
            a.name().cmp(&b.name())
        });
        Ok(res)
    }
    
}

