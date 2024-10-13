use clap::Parser;
use std::path::Path;
use std::fs;
use std::collections::HashMap;
use std::io::Error;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    /// Path your TODOs reside in
    #[arg(short, long)]
    path: String
}

#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to find TODOs for 
    #[arg(short, long)]
    name: String,

    #[arg(short, long)]
    users: Vec<String>

}

#[derive(Subcommand)]
enum Command {
    /// Creates a new project or category
    New,
}

struct Todo {
    dir_name: String,
    task: String
}

impl Todo {
    pub fn new (dir_name: String, task:String) -> Todo {
        Todo {
            dir_name,
            task
        }
    }
}


fn search_directory(path: Box<&Path>, file_name: Box<&String>) -> Result<Vec<Todo>, Error> {
    let mut todos = Vec::with_capacity(10);
    for entry in path.read_dir().expect("Could not read the directory") {
        if let Ok(entry) = entry {
            let new_path = entry.path();
            if new_path.is_dir() {
                // enter the dir and do the same 
                let mut more_todos = search_directory(Box::new(new_path.as_path()), file_name.clone())?;//.expect("Inner directory error");
                todos.append(&mut more_todos);
            } else {

                if let Some(new_file_name) = new_path.file_name() {
                    if let Some(new_file_name_str) = new_file_name.to_str() {
                        if &*file_name.as_str() == new_file_name_str {
                            if let Some(path_str) = new_path.to_str() {
                                let contents: String = fs::read_to_string(path_str)?;//.expect("File could not be read");
                                for line in contents.lines() {
                                    if line.starts_with("-") {
                                            if line.get(..4) != Some("- ~~") {
                                                let components = path_str.rsplit("/").collect::<Vec<_>>();
                                                let new_todo = Todo::new(String::from(components[1]), String::from(line));
                                                todos.push(new_todo);
                                            }
                                        }
                                }


                            }
                        }
                    }
                }
            }
        }
    }
    Ok(todos)
}

fn create_directory(new_dir_path: Box<&Path>, users: Box<Vec<String>>) -> Result<(), Error> {
    fs::create_dir(new_dir_path)?;
    fs::write("list.md", "# Todos")?;

    // for each user provided, create a list
    for user in users.iter() {
        let file_name = user.push_str(".md");
        fs::write(file_name, "# Todos")?;
    }
    
    Ok()
}

fn print_lists(path_str: Box<&str>, name_str: Box<&str>) -> Result<(), Error> {

    let path_string = String::from(path_str);
    if !path_string.is_empty() {
        let mut file_name = String::from(name_str);

        file_name.push_str(".md");

        if let Ok(list) = search_directory(Box::new(path), Box::new(&file_name)) {

            let mut projects = HashMap::with_capacity(10);
            for item in list.iter() {
                if !projects.contains_key(&item.dir_name) {
                    projects.insert(item.dir_name.clone(), Vec::with_capacity(6));
                } else {
                    if projects.contains_key(&item.dir_name) {
                        if let Some(project_mut) = projects.get_mut(&item.dir_name) {
                            project_mut.push(item.task.clone())
                        }
                    }
                }
            }

            println!("\n\n");
            for (key, val) in projects.iter() {
                println!("\n");
                let key = key.replace("_", " ");
                println!("{}", key.to_uppercase());
                println!("-------------------------------");
                for task in val.iter() {
                    println!("{}", task);
                }
            }
            println!("\n\n");


        } else {
            println!("There was an error accessing the project files");
        }
    }

    Ok()

}

fn main() {
    let cli = Cli::parse();

    let args = Args::parse();

    let path = Path::new(&cli.path);

    let name_str = &args.name.to_string();
    let path_str = path.to_str().expect("Path not a string");

    // example of cli modes
    match cli.command {
        Command::New => {
            // creates a new project
            create_directory(&path, &args.users);

        }
        _ => {
            print_lists(&path_str, &name_str);

        }
    }


}
