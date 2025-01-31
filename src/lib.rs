use crate::todo::*;
use crate::Config;

// file in ~/todos.txt

pub mod todo {
    use std::{fs};

    pub fn run(conf: Config){
        show_todos();
    }

    pub struct Config {
        pub command: String,
        pub arg: String,
    }
    
    pub struct Todo {
        id: u32,
        text: String,
        ticked: bool,
    }
    
    struct TodoList {
        pub todos: Vec<Todo>,
    }
    
    //impl Display for Todo

    pub fn persist_todo(todo: Todo) {
        // check for file
        
        // write to file
    }

    pub fn show_todos() {
        // read file
        let content = match fs::read_to_string("/Users/aljoschalabonte/todos.txt"){
            Ok(text) => {
                println!("Filecontent: {text}");
                text
            },
            Err(err) => { 
                panic!("File not read... {err}"); 
            },
        };

        let mut todos: Vec<String> = Vec::new();
        let mut counter = 0;
        for index in (0..content.len()-1)  {
           if content.chars().nth(index).unwrap() == "\n".chars().nth(0).unwrap() {
            let line = String::from(&content[counter..index]);
            todos.push(line);
            counter = index + 1;
           } 
        }

        dbg!(&todos);

        let mut todolist: Vec<Todo> = vec![];

        let mut counter = 0;
        for td in todos {
            let mut values: Vec<String> = Vec::new();
            for index in (0..td.len()-1) {
                if td.chars().nth(index).unwrap() == ",".chars().nth(0).unwrap() {
                    let mut value = String::from(&td[counter..index]);
                    values.push(value);
                    counter = index + 1;
                } 
            }
            let mut value = String::from(&td[counter..td.len()]);
            values.push(value);

            dbg!(&values);

            let id: u32 = values.get(0).expect("ID should be in line").trim().parse().unwrap(); //u32::from(values.get(0).expect("ID should be in line"));
            let text = String::from(values.get(1).expect("text should be in line"));
            let ticked: bool = values.get(2).expect("status should be in line").trim().parse().unwrap();

            let mut todo = Todo { id, text, ticked };
            todolist.push(todo)
        }

        //parse_todos(&content);
            
        // create Todos from file content
        // iterate and print on stdout

    }

    pub fn parse_todos(stream: &String) -> &'static str {
        let mut todos = stream.split("\n");
        dbg!(todos);
        "hello"
    }
    /*pub fn show_todo(id: u32) -> Todo {
        // read file
        // create Todo from file content
        // print on stdout
    }

    pub fn tick(id: u32) -> Todo {
        // read todo from file
        // verify status
        // tick todo
    }

    pub fn delete_todo(id: u32) -> Todo {

    }*/
}
