use clap::Parser;
use std::path::Path;
use std::fs;
use std::collections::HashMap;
use std::io::Error;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to find TODOs for 
    #[arg(short, long)]
    name: String,

    /// Path your TODOs reside in
    #[arg(short, long)]
    path: String

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
                                    match line.get(..2) {
                                        Some("- ") => { 
                                            if line.get(..4) != Some("- ~~") {
                                                let components = path_str.rsplit("/").collect::<Vec<_>>();
                                                let new_todo = Todo::new(String::from(components[1]), String::from(line));
                                                todos.push(new_todo);
                                            }
                                        },
                                        Some(_) => {},
                                        None => {} 
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


fn main() {
    let args = Args::parse();
    let name_str = &args.name.to_string();
    let path = Path::new(&args.path);
    let path_str = path.to_str().expect("Path not a string");
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



}
