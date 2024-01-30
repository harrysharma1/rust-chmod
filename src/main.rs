mod chmod;
use chmod::Chmod;
use clap::Parser;

#[derive(Parser)]
struct  Cli{
   pattern:String,
   value: std::path::PathBuf,
}


fn main() {
    let args = Cli::parse();
    let chmod = Chmod{};
    match args.pattern.as_str() {
        "o"|"octal_to_symbolic"=>{
            let value  = args.value.to_str();
            match value {
                Some(_)=>{
                    let val_str = value.unwrap();
                    match val_str.parse::<u16>(){
                        Ok(val_int) => {
                            print!("{}",chmod.convert_octal_to_symbolic(val_int));
                            
                        },
                        Err(_) => println!("<Err: Not a positive integer>"),
                    }
                    
                },
                None=>println!("<Err: Value not recognised due to parsing error>"),
                
            }
          
        },
        "s"|"symbolic_to_octal"=>{
            let value = args.value.to_str();
            match value{
                Some(_)=>{
                    let val_str = value.unwrap();
                    println!("{}",chmod.convert_symbolic_to_octal(val_str.to_string()));
                },
                None=>println!("<Err: Value not recognised due to parsing error>"),
            }

        },
        _=>print!(""),

        
    }

    
    
}
