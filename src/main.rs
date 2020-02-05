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
    let file_names: Vec<String> = get_file_names(path);
    println!("{:?}", file_names);
    for file_name in file_names.iter()
    {
        let mut underscore_count: u8 = 0;
        let mut us_pos: Vec<usize> = Vec::with_capacity(2);

        for c in file_name.char_indices().rev()
        {
            if c.1 == '_'
            {
                underscore_count += 1;
                us_pos.push(c.0);
            }

            if underscore_count == 2
            {
                if let Some(uploader) = file_name.get(us_pos[1] + 1..us_pos[0])
                {
                    println!("{}", uploader);
                }
                break;
            }
            //match c
            //{
                // (index, '_') => underscore_count += 1; underscore_indices.push(index)),
                // _ => (),
            // }
        }
    }
}

fn get_file_names(path: String) -> Vec<String>
{
    let mut file_names: Vec<String> = Vec::new();
    if let Ok(entries) = fs::read_dir(&path)
    {
        for entry in entries
        {
            if let Ok(entry) = entry
            {
                match entry.file_name().into_string()
                {
                    Ok(file_name) => file_names.push(file_name),
                    Err(_) => println!("Error converting OsString to String."),
                }
            }
        }
    }
    else
    {
        println!("The path: {} could not be read.", path)
    }

    file_names
}