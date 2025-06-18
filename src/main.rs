use std::env;
use std::fs;
fn main() {
    //refactor read from file
    let args:Vec<String>=env::args().collect();

    let config=Config::new(&args);
    println!("{:?}",config.query);
    println!("{:?}",config.filename);

    let contents =fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    println!("{}",contents);
    
}

struct Config{
    query:String,
    filename:String
}
impl Config{
    fn new(args:&[String])-> Config {
        if args.len()<3{
            panic!("please provide query and filename");
        }
    let query:String=args[1].clone();
    let filename:String=args[2].clone();
    Config{query,filename}
}
}



