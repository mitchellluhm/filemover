use std::fs;
use std::env;

fn main()
{
    let args: Vec<String> = env::args().collect();
    if args.len() > 1
    {
        run(args[1].clone());
    }
    else
    {
        println!("Please give the program a command line argument for the path.");
    }
}
    
fn run(path: String)
{
    if let Ok(entries) = fs::read_dir(&path)
    {
        for entry in entries
        {
            if let Ok(entry) = entry
            {
                println!("{:?}", entry.file_name());
            }
        }
    }
    else
    {
        println!("The path: {} could not be read.", path)
    }

}