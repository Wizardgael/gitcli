use std::process::{Command, Output};




pub struct Git{

}

impl Git{

    fn command(com: String, arg: Vec<String>) -> Output{
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
        command.args(arg);

        command.output()
        .expect("Error during command")
    }

    pub fn hello_world(&self){
        let output = Git::command("echo".into(), vec!["Hello World!".into()]);
        println!("{:?}", output);
        println!("{:?}", output.stdout);
    }
    
}
