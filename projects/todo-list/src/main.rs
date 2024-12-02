use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::vec;

struct Todo {
    index: usize,
    task: String,
    completed: bool,
}

struct TodoList {
    todolist: Vec<Todo>,
}

impl TodoList {
    fn new() -> Self {
        TodoList {
            todolist: Vec::new(),
        }
    }

    fn add(&mut self, task: String) {
        let index = self.todolist.len() as usize;
        let new_task = Todo {
            index,
            task,
            completed: false,
        };
        self.todolist.push(new_task);
        println!("Task Added!")
    }

    fn list_tasks(&self) {
        if self.todolist.len() > 0 {
            for todo in &self.todolist {
                println!(
                    "[{}] {}: {}",
                    if todo.completed { "✓" } else { " " },
                    todo.index + 1,
                    todo.task,
                );
            }
        } else {
            println!("No task added yet!")
        }
    }

    fn complete(&mut self, index: usize) {
        if let Some(items) = self.todolist.iter_mut().find(|i| i.index == index) {
            items.completed = true;
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();

    let items = vec!["Add task", "List tasks", "Complete the task", "Exit"];
    loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What do you choose?")
            .items(&items)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => {
                let task: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter the name of task")
                    .interact_text()
                    .unwrap();
                todo_list.add(task);
            }
            1 => todo_list.list_tasks(),
            2 => {
                let todos = todo_list
                    .todolist
                    .iter()
                    .map(|items| format!("{}) {}", items.index + 1, items.task))
                    .collect::<Vec<String>>();

                let selection = Select::new()
                    .with_prompt("What do you choose?")
                    .items(&todos)
                    .default(0)
                    .interact()
                    .unwrap();

                todo_list.complete(selection);
            }
            3 => {
                println!("Thanks for using the tool!");
                break;
            }
            _ => todo!(),
        }
    }
}
