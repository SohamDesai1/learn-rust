use dialoguer::{theme::ColorfulTheme, Input, Select};
struct Expense {
    index: usize,
    title: String,
    amount: f32,
}

struct Budget {
    expense: Vec<Expense>,
}

impl Budget {
    fn new() -> Budget {
        Budget {
            expense: Vec::new(),
        }
    }

    fn add_expenses(&mut self, title: String, amount: f32) {
        let index = self.expense.len() as usize;
        let new_item = Expense {
            index,
            title,
            amount,
        };

        self.expense.push(new_item);

        println!("Item Added!")
    }

    fn edit_expense(&mut self, index: usize, new_title: Option<String>, new_amount: Option<f32>) {
        if let Some(expense) = self.expense.get_mut(index) {
            if let Some(name) = new_title {
                expense.title = name;
            }
            if let Some(amount) = new_amount {
                expense.amount = amount;
            }
            println!("Expense updated successfully!");
        } else {
            println!("Expense not found");
        }
    }

    fn list_expenses(&self) {
        if self.expense.len() > 0 {
            for expense in &self.expense {
                println!(
                    "{}) {} : {}₹",
                    expense.index + 1,
                    expense.title,
                    expense.amount
                )
            }
        } else {
            println!("No task added yet!")
        }
    }
}

fn main() {
    let mut budget = Budget::new();

    let options = vec!["Add Expense", "List Budget", "Edit Expense", "Exit"];

    println!("Welcome to Budget Manager!");
    loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What do you choose?")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => {
                let title: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter the name of the item")
                    .interact_text()
                    .unwrap();
                loop {
                    let amount: String = Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Enter the amount")
                        .interact_text()
                        .unwrap();

                    match amount.trim().parse::<f32>() {
                        Ok(parsed_amount) => {
                            budget.add_expenses(title, parsed_amount);
                            break;
                        }
                        Err(_) => {
                            println!("Invalid input. Please enter a valid number for the amount.");
                        }
                    }
                }
            }
            1 => budget.list_expenses(),
            2 => {
                let expenses = budget
                    .expense
                    .iter()
                    .map(|items| format!("{} : {}₹", items.title, items.amount))
                    .collect::<Vec<String>>();

                if expenses.is_empty() {
                    println!("No expenses to edit.");
                    continue;
                }
                let selection = Select::new()
                    .with_prompt("Which one would you like to edit?")
                    .items(&expenses)
                    .default(0)
                    .interact()
                    .unwrap();

                let new_title: Option<String> = {
                    let input: String = Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Enter new name (press Enter to keep current)")
                        .allow_empty(true)
                        .interact_text()
                        .unwrap();

                    if input.trim().is_empty() {
                        None
                    } else {
                        Some(input)
                    }
                };

                let new_amount: Option<f32> = {
                    let input: String = Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Enter new amount (press Enter to keep current)")
                        .allow_empty(true)
                        .interact_text()
                        .unwrap();

                    if input.trim().is_empty() {
                        None
                    } else {
                        match input.trim().parse::<f32>() {
                            Ok(amount) => Some(amount),
                            Err(_) => {
                                println!("Invalid amount entered. Keeping current value.");
                                None
                            }
                        }
                    }
                };

                budget.edit_expense(selection, new_title, new_amount);
            }
            3 => {
                println!("Thank you for using the tool!");
                break;
            }
            _ => todo!(),
        }
    }
}
