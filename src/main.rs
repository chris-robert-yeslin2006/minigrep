use std::env;
use std::fs;
use std::process;
use std::error::Error;
fn main() {
    //refactor read from file
    let args:Vec<String>=env::args().collect();
    //this unwarap fn dont crash the pgrm but exit with code 1
    let config=Config::new(&args).unwrap_or_else(|err|{
        println!("problme parsing arguments:{}",err);
        process::exit(1);
    });
    println!("{:?}",config.query);
    println!("{:?}",config.filename);

    if let Err(e)=run(config){
        print!("Application error: {}", e);
        process::exit(1);
    }

    
    
}

//Box<dyn Error> to allow any kind of error type-wrapped
fn run(config:Config)->Result<(),Box<dyn Error>>{
    let contents =fs::read_to_string(config.filename)?;

    println!("{}",contents);

    Ok(())
}
struct Config{
    query:String,
    filename:String
}
impl Config{
    fn new(args:&[String])->  Result<Config,&str> {
        if args.len()<3{
           return Err("not enougn args");
        }
    let query:String=args[1].clone();
    let filename:String=args[2].clone();
    Ok(Config{query,filename})
}
}



