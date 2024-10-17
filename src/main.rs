use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;
use std::io::Error;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    
    /// Path your TODOs reside in
    path: Option<String>,

    name: Option<String>,

    users: Option<Vec<String>>,

}

//#[(version, about, long_about = None)]
/*
struct Args {
    /// Name of the person to find TODOs for 
    #[arg(short, long)]
    name: String,

    #[arg(short, long)]
    users: Vec<String>

}
*/

#[derive(Subcommand, Debug)]
enum Commands {
    /// Creates a new project or category
    New {
        #[arg(short, long)]
        project: Option<String>,

    },
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
            // this is a PathBuf type
            let new_path = entry.path();
            if new_path.is_dir() {
                // enter the dir and do the same 
                let mut more_todos = search_directory(Box::new(new_path.as_path()), file_name.clone())?;//.expect("Inner directory error");
                todos.append(&mut more_todos);
            } else {
                // if the file names match
                if let Some(new_file_name) = new_path.file_name() {
                    if let Some(new_file_name_str) = new_file_name.to_str() {
                        if &*file_name.as_str() == new_file_name_str {
                            if let Some(path_str) = new_path.to_str() {
                                let contents: String = fs::read_to_string(path_str)?;
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
                } else if let Some(found_file_str) = new_path.to_str() {
                    // if the file is a markdown file, then open it
                    if found_file_str.ends_with(".md") {
                        // TODO: this is copied verbosely, so find a way to remove duplication
                        let contents: String = fs::read_to_string(found_file_str)?;
                        for line in contents.lines() {
                            if line.starts_with("-") {
                                    if line.get(..4) != Some("- ~~") {
                                        let components = found_file_str.rsplit("/").collect::<Vec<_>>();
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
    Ok(todos)
}

fn create_directory(new_dir_path: Box<&Path>, users: Box<Vec<String>>) -> Result<(), Error> {
    // Creates a route to the absolute path
    let abs_path = PathBuf::from(*new_dir_path);
    let abs_path_can = fs::canonicalize(&abs_path)?;
    // Create a directory at the specified path
    fs::create_dir(abs_path_can)?;
    // Creates the path for the main list
    let mut main_list_path = PathBuf::from(&abs_path);
    main_list_path.push("list.md");
    let main_list_path_can = fs::canonicalize(&main_list_path)?;
    fs::write(main_list_path_can, "# Todos")?;

    // for each user provided, create a list
    for user in users.iter() {
        let user = user.as_str();
        let file_name = String::from(user);
        let mut file_path_buf = PathBuf::from(&abs_path);
        file_path_buf.push(&file_name);
        file_path_buf.push(".md");
        fs::write(file_path_buf, "# Todos")?;
    }
    
    Ok(())
}

fn print_lists(path_str: Box<&str>, name_str: Box<&str>) -> Result<(), Error> {
     
    //let path_string = String::from(*path_str);
    let path = Path::new(*path_str);
    println!("{}", path.display());
    let mut file_name = String::from(*name_str);

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

    Ok(())

}

fn main() {
    let cli = Cli::parse();
    
    //let args = Args::parse();
       // example of cli modes
    match &cli.command {
        Some(Commands::New { project }) => {
            // creates a new project
            //create_directory(&path, &args.users);
            

        }
        None => {
            if let Some(path) = cli.path {
                let path = Path::new(&path);
                let path_str = path.to_str().expect("Path not a string");
                if let Some(name_str) = cli.name {
                    let name_str = name_str.as_str();
                    print_lists(Box::new(path_str), Box::new(name_str));
                } else {
                    print_lists(Box::new("./"), Box::new(""));
                }

            } else {
                print_lists(Box::new("./"), Box::new(""));
            }

        }
    }


}
