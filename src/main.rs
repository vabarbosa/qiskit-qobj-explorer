use std::env;

mod qobj;
use qobj::Qobj;

fn usage(program_name: &String){
    println!("Usage:\n");
    println!("{} <qobj>\n", program_name);
    ::std::process::exit(-1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage(&args[0]);
    }

    let qobj_data = &args[1];
    let qob: Qobj = serde_json::from_str(qobj_data).expect("Couldn't load QObj");


}