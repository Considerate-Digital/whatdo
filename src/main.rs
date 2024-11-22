use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
use std::{fs, process};
use std::collections::HashMap;
use std::io::{Error, Write};


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct SubCli {
    name: Option<String>,
    
    /// Path your TODOs reside in
    #[arg(short, long,)]
    path: Option<String>,

    #[arg(short, long, value_delimiter=',')]
    users: Option<Vec<String>>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// This command creates a new project or category
    New {
        project_name: Option<String>,

        #[arg(short, long)]
        path: Option<String>,

        #[arg(short, long, value_delimiter=',')]
        users: Option<Vec<String>>,
    },
    List(SubCli),
    Add {
        project_name: Option<String>,

        #[arg(short, long)]
        user: Option<String>,

        #[arg(short, long)]
        path: Option<String>,

        
        todo: Option<String>
    }
}

#[derive(Debug)]
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
                let mut more_todos = search_directory(Box::new(new_path.as_path()), file_name.clone())?;
                todos.append(&mut more_todos);
            } else {
                // if the file names match
                if let Some(new_file_name) = new_path.file_name() {
                    if let Some(new_file_name_str) = new_file_name.to_str() {
                        //println!("{}", file_name.as_str());
                        //println!("{}", new_file_name_str);
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
                        } else if &*file_name.as_str() == ".md" {
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

fn create_directory(new_dir_path: Box<&Path>, users: Box<Vec<String>>) -> Result<(), Error> {
    // Creates a route to the absolute path
    let abs_path = PathBuf::from(*new_dir_path);

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
        let file_name = String::from(user);
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
    
    // give the file name the correct extension
    file_name.push_str(".md");

    if let Ok(list) = search_directory(Box::new(path), Box::new(&file_name)) {

        let mut projects = HashMap::with_capacity(10);
        for item in list.iter() {
            if !projects.contains_key(&item.dir_name) {
                projects.insert(item.dir_name.clone(), Vec::with_capacity(6));
                if let Some(project_mut) = projects.get_mut(&item.dir_name) {
                    project_mut.push(item.task.clone())
                }

            } else if projects.contains_key(&item.dir_name) {
                if let Some(project_mut) = projects.get_mut(&item.dir_name) {
                    project_mut.push(item.task.clone())
                }
            }
        }

        //println!("\n\n");

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
    if let Some(name_str) = &cli.name {
        let name_str = name_str.as_str();
        if let Some(path) = &cli.path {
            let path = Path::new(&path);
            let path_str = path.to_str().expect("Path not a string");
            print_lists(Box::new(path_str), Box::new(name_str))?;
        } else {
            print_lists(Box::new("./"), Box::new(name_str))?;
        }
    } else {
        if let Some(path) = &cli.path {
            let path = Path::new(&path);
            let path_str = path.to_str().expect("Path not a string");
            print_lists(Box::new(path_str), Box::new(""))?;
        } else {
            print_lists(Box::new("./"), Box::new(""))?;
        }
    }
    Ok(())
}


fn add_todo(project_name: Box<&String>, user: Box<&String>, path: Box<&String>, todo: Box<&String>) -> Result<(), Error> {

    let mut path = PathBuf::from(*path);
    path.push(*project_name);

    path.push(*user);
    path.set_extension("md");

    let mut file = fs::OpenOptions::new()
            .write(true)
        .append(true)
        .open(&path)?;
    
    let mut new_todo = String::from("- ");
    new_todo.push_str(&*&todo);

    if let Err(e) = writeln!(file, "{}", &new_todo) {
        eprintln!("Could not write to file: {}", e)
    }

    println!("Todo \"{}\" added to file: \"{:?}\"", &todo, &file);


    Ok(())
}

fn main() {
    let cli = Cli::parse();
    
    // example of cli modes
    match &cli.command {
        Some(Commands::New { project_name, path, users }) => {
            // creates a new project
            //create_directory(&path, &args.users);
            // make the new path name from the project name and args
            if let Some(project_name) = &project_name {
                if let Some(new_path) = &path {
                    let mut path = String::from("./");
                    path.push_str(&new_path);
                    path.push_str("/");
                    path.push_str(&project_name);
                    let path = Path::new(&path);
                    
                    if let Some(users) = &users {
                        if let Err(e) = create_directory(Box::new(&path), Box::new(users.to_vec())) {
                            eprintln!("New project could not be created: {}", e);
                            process::exit(1);
                        }
                    } else {
                        if let Err(e) = create_directory(Box::new(&path), Box::new(Vec::new())) {
                            eprintln!("New project could not be created: {}", e);
                            process::exit(1);
                        }
                    }
                } else {
                    let mut path = String::from("./");
                    path.push_str(&project_name);
                    let path = Path::new(&path);
                    if let Some(users) = &users {
                        if let Err(e) = create_directory(Box::new(&path), Box::new(users.to_vec())) {
                            eprintln!("New project could not be created: {}", e);
                            process::exit(1);
                        }
                    } else {
                        if let Err(e) = create_directory(Box::new(&path), Box::new(Vec::new())) {
                            eprintln!("New project could not be created: {}", e);
                            process::exit(1);
                        }
                    }
                }
            }

        }
        Some(Commands::List(args)) => {
            if let Err(e) = print_sorter(Box::new(&args)) {
                eprintln!("List could not be displayed: {}", e);
                process::exit(1);
            }
        }
        Some(Commands::Add{project_name, user, path, todo}) => {

            if let Some(project_name) = &project_name {
                if let Some(todo) = &todo {
                    if let Some(user) = &user {
                        if let Some(path) = &path {
                            if let Err(e) = add_todo(Box::new(project_name), Box::new(user), Box::new(path), Box::new(todo)) {
                                eprintln!("Could not add todo: {}", e);
                                process::exit(1);
                            }

                        } else {
                            if let Err(e) = add_todo(Box::new(project_name), Box::new(user), Box::new(&String::new()), Box::new(todo)) {

                            eprintln!("Could not add todo: {}", e);
                            process::exit(1);
                            }
                        }
                    } else if let Some(path) = &path  {
                        if let Err(e) = add_todo(Box::new(project_name), Box::new(&String::new()), Box::new(path), Box::new(todo)) {
                            eprintln!("Could not add todo: {}", e);
                            process::exit(1);

                        }
                    } else {
                        if let Err(e) = add_todo(Box::new(project_name), Box::new(&String::new()), Box::new(&String::new()), Box::new(todo)) {
                            eprintln!("Could not add todo: {}", e);
                            process::exit(1);
                        }
                    }
                } else {
                    eprintln!("Please include a \"todo\" you would like adding to the project");
                    process::exit(1);
                }
            } else {
                eprintln!("Please specify a project to add a \"todo\" to");
                process::exit(1);
            }
        }
        None => {
            eprintln!("Please specify a command");
            process::exit(1);
        }
    }
}
