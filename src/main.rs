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
}

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct SubCli {
    /// Path your TODOs reside in
    path: Option<String>,

    name: Option<String>,
    
    users: Option<Vec<String>>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Creates a new project or category
    New {
        project_name: Option<String>,

        #[arg(short, long)]
        path: Option<String>,

        #[arg(short, long, value_delimiter=',')]
        users: Option<Vec<String>>,
    },
    List(SubCli),
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
                            println!("{}", new_file_name_str);
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
                        } else if &*file_name.as_str() == " " {
                            if let Some(found_file_str) = new_path.to_str() {
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
            }
        }
    }
    Ok(todos)
}

fn create_directory(new_dir_path: Box<&Path>, mut users: Box<Vec<String>>) -> Result<(), Error> {
    println!("create directory called");
    println!("{:?}", users);
    // Creates a route to the absolute path
    let abs_path = PathBuf::from(*new_dir_path);
    println!("{}",abs_path.display());

    /*
    println!("calling canonicalize");
    let abs_path_can = fs::canonicalize(&abs_path)?;
    println!("{}", abs_path_can.display());
    */
    // Create a directory at the specified path
    fs::create_dir(&abs_path)?;
    // Creates the path for the main list
    let mut main_list_path = PathBuf::from(&abs_path);
    main_list_path.push("general.md");

    fs::write(&main_list_path, "# Todos \n - Example Todo")?;
    
    // for each user provided, create a list
    for user in users.iter() {
        // make the string lowercase
        let user = user.to_lowercase();
        let user = user.as_str();
        let mut file_name = String::from(user);
        //file_name.push_str(".md");
        let mut file_path_buf = PathBuf::new();
        file_path_buf.push(&abs_path);
        file_path_buf.push(&file_name);
        file_path_buf.set_extension("md");

        fs::write(file_path_buf, "# Todos \n - Example Todo")?;
    }
    
    Ok(())
}

fn print_lists(path_str: Box<&str>, name_str: Box<&str>) -> Result<(), Error> {
    let path = Path::new(*path_str);
    let mut file_name = String::from(*name_str);

    file_name.push_str(".md");

    if let Ok(list) = search_directory(Box::new(path), Box::new(&file_name)) {

        let mut projects = HashMap::with_capacity(10);
        for item in list.iter() {
            if !projects.contains_key(&item.dir_name) {
                projects.insert(item.dir_name.clone(), Vec::with_capacity(6));
            } else if projects.contains_key(&item.dir_name) {
                if let Some(project_mut) = projects.get_mut(&item.dir_name) {
                    project_mut.push(item.task.clone())
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

fn print_sorter(cli: Box<&SubCli>) -> Result<(), Error> {
    if let Some(path) = &cli.path {
        let path = Path::new(&path);
        let path_str = path.to_str().expect("Path not a string");
        if let Some(name_str) = &cli.name {
            let name_str = name_str.as_str();
            print_lists(Box::new(path_str), Box::new(name_str));
            Ok(())
        } else if path_str != "" {
            print_lists(Box::new(path_str), Box::new(""));
            Ok(())
        } else {
            print_lists(Box::new("./"), Box::new(""));
            Ok(())
        }
    } else {
        print_lists(Box::new("./"), Box::new(""));
        Ok(())
    }
}

fn main() {
    let cli = Cli::parse();
    
    //let args = Args::parse();
       // example of cli modes
    match &cli.command {
        Some(Commands::New { project_name, path, users }) => {
            // creates a new project
            //create_directory(&path, &args.users);
            // make the new path name from the project name and args
            if let Some(project_name) = &project_name {
                if let Some(new_path) = &path {
                    println!("path found");
                    let mut path = String::from("./");

                    path.push_str(&new_path);
                    path.push_str("/");
                    path.push_str(&project_name);
                    let path = Path::new(&path);
                    
                    if let Some(users) = &users {
                        create_directory(Box::new(&path), Box::new(users.to_vec()));
                    } else {
                        create_directory(Box::new(&path), Box::new(Vec::new()));
                    }
                } else {
                    println!("no path found");
                    let mut path = String::from("./");
                    path.push_str(&project_name);
                    let path = Path::new(&path);
                    println!("{}", path.display());
                    if let Some(users) = &users {
                        create_directory(Box::new(&path), Box::new(users.to_vec()));
                    } else {
                        create_directory(Box::new(&path), Box::new(Vec::new()));
                    }
                }
            }

        }
        Some(Commands::List(args)) => {
            print_sorter(Box::new(&args));    
        }
        None => {
            //print_sorter(Box::new());    

        }
    }


}
