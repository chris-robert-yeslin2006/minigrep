use std::env;

fn main() {
    //read from command line of query and filename
    let args:Vec<String>=env::args().collect();

    let query: &String=&args[1];
    let filename:&String=&args[2];
    println!("{:?}",query);
    println!("{:?}",filename);
    
}
