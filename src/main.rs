use std::{io, io::Write};

fn main() {
    let task_0 = Task::new();
    
    println!("\n++Your task++");
    print!("Title: {}\nDetail: {}", task_0.task_title, task_0.task_detail);
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
        input_title = Self::input(1);

        'task_detail: loop {
            print!("Please input your task detail: ");
            input_detail = Self::input(2);

            if input_detail.is_empty() {
                println!("\nDo you want to store the task without detail? (y/n)");
                loop {
                    let mut yes_no = String::new();
                    keyboard_input(&mut yes_no);

                    match yes_no.to_lowercase().trim() {
                       "y" => break 'task_detail,
                       "n" => break,
                       _ => { println!("Please type either 'y' or 'n'"); continue },
                    }
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
    
    fn input(task_field: i32) -> String {
        let mut input = String::new();

        while input.is_empty() {
            keyboard_input(&mut input);

            if input.trim().is_empty() && task_field == 2 {
                input.clear();
                break;
            } else if input.trim().is_empty() {
                input.clear();
                print!("Input is empty!\n\nPlease input your task title: ");
            } else {
                input = input.trim().to_string();
            }
        }
        input
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_test() {
        let task_title = String::from("\n");

        assert_eq!(task_title.trim(), "");
    }
}
