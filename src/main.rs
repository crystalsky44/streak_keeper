use std::io;

fn main() {
    println!("Please input your task: ");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Task title is required to add task");

    println!("Task added: {}", input.trim());
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_test() {
        let task_title = 
*/
