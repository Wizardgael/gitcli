use std::process::{Command, Output};
use std::str;



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


    pub fn untracked_file(&self) -> Result<Vec<String>, String>{
        let output = Git::command("git ls-files . --exclude-standard --others".into());
        if let Ok(s) = String::from_utf8(output.stdout) {
            Ok(s.trim().split("\n").map(|f| f.to_string()).collect())
        }else {
            return Err("Error reading untracked files".into())
        }
    }
    
}
