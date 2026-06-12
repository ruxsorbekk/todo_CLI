use std::io;

#[derive(Debug)]
struct ToDo {
    id: u32,
    name: String,
    completed: bool,
}
struct TodoList {
    todos: Vec<ToDo>,
    next_id: u32,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList { 
            todos: Vec::new(),
            next_id: 1 
        }
    }
    fn add(&mut self, name: String) {
        self.todos.push(
            ToDo { 
                id: self.next_id,
                name,
                completed: false }
        );
        self.next_id += 1;
    }
    fn read(&self) {
        if self.todos.is_empty() {
            println!("=================");
            println!("No tasks yet!");
            println!("=================");
            return;
        }
        println!("=================");
        println!("All tasks: ");
        for todo in self.todos.iter() {
            println!("{}. {}: status - {}", todo.id, todo.name, todo.completed);
        }
        println!("=================")
    }
    fn complete(&mut self, id: u32) {
        for todo in &mut self.todos {
            if todo.id == id {
                todo.completed = true;
                println!("=================");
                println!("Completed: {}", todo.name);
                println!("=================");
                return;
            }
        }
        println!("Task not found!");

    }
    fn delete(&mut self, id: u32) {
        let len_before = self.todos.len();
        self.todos.retain(|t| t.id != id);
        if self.todos.len() < len_before {
            println!("=================");
            println!("Deleted task {}", id);
            println!("=================");
        } else {
            println!("Task {} not found!", id);
            println!("=================");
        }
    }
}

fn menu() {
    println!("1. Add task");
    println!("2. All tasks");
    println!("3. Complete task");
    println!("4. Delete task");
    println!("5. Quit");
    println!(">>> ");
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let mut todos = TodoList::new();
    
    loop {
        menu();
        
        let choice = read_line();
        match choice.as_str() {
            "1" => {
                println!("=================");
                println!("Enter task title: ");
                let title = read_line();
                todos.add(title);
                println!("=================");
            }
            "2" => todos.read(),
            "3" => {
                println!("=================");
                println!("Enter task ID to complete: ");
                if let Ok(id) = read_line().parse::<u32>() {
                    todos.complete(id);
                }
                println!("=================");
            }
            "4" => {
                println!("=================");
                println!("Enter task ID to delete: ");
                if let Ok(id) = read_line().parse::<u32>() {
                    todos.delete(id);
                }
                println!("=================");
            }
            "5" => {
                println!("Exiting...");
                break;
            },
            &_ => todo!()
        }
    }
    
}