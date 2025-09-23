use std::{io, io::Write};

fn main() {
    let task_0 = run();
    
    println!("\n++Your task++");
    print!("Title: {}\nDetail: {}", task_0.task_title, task_0.task_detail);
}

fn run() -> Task {
    let task = Task::new();
    task
}

fn keyboard_input(input: &mut String) -> () {
    // flush output
    io::stdout()
        .flush()
        .expect("just_for_test");

    io::stdin()
        .read_line(input)
        .expect("Task title is required to add task {data_name}");
}

struct Task {
    task_title: String,
    task_detail: String,
}
 
impl Task {
    fn new() -> Task {
        let input_title;
        let mut input_detail;

        print!("Please input your task title: ");
        input_title = Self::input("title".to_string());

        loop {
            print!("Please input your task detail: ");
            input_detail = Self::input("detail".to_string());

            if input_detail.is_empty() {
                println!("\nDo you want to store the task without detail? (y/n)");
                let mut yes_no = String::new();
                keyboard_input(&mut yes_no);

                match yes_no.to_lowercase().trim() {
                   "y" => break,
                   "n" => continue,
                   _ => { println!("Please type either 'y' or 'n'"); continue },
                }
            } else {
                break;
            }
        }

        let task_0 = Task {
            task_title: input_title,
            task_detail: input_detail,
        };

        task_0
 
    }
    
    fn input(data_name: String) -> String {
        let mut input = String::new();

        while input.is_empty() {
            keyboard_input(&mut input);

            if input.trim().is_empty() && data_name == "detail" {
                input.clear();
                break;
            } else if input.trim().is_empty() {
                input.clear();
                print!("Input is empty!\n\nPlease input your task {data_name}: ");
            } else {
                input = input.trim().to_string();
            }
        }
        input
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_test() {
        let task_title = String::from("\n");

        assert_eq!(task_title.trim(), "");
    }
}
