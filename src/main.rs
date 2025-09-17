use std::io;
use std::io::Write;

fn main() {
    print!("Please input your task: ");
    io::stdout().flush().expect("just_for_test");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Task title is required to add task");

    while input.trim().is_empty() {
        print!("Input is empty!\nPlease input your task: ");
        io::stdout().flush().expect("just_for_test");
        io::stdin().read_line(&mut input)
            .expect("Task title is required to add task");
    } 

    println!("Task added: {}", input.trim());
    
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
