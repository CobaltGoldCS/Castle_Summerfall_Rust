use std::io::{stdout, Write, stdin};

fn main(){
    let stdin = stdin();

    loop {
        let mut input = String::from("");

        print!("> ");
        stdout().flush().unwrap();

        stdin.read_line(&mut input).expect("Failed to read line");
        input = input.to_string();
        input = (&mut input).trim().to_string();

        if input.eq("exit"){
            break;
        }
    }
}