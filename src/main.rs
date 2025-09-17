use std::io;

fn main() {
    println!("Please input your task: ");

    let mut input = String::new();

    let test_usize: usize = io::stdin().read_line(&mut input)
        .expect("Task title is required to add task");

    println!("Task added: {}", input.trim());
    println!("test_my_knowledge: {}", test_usize);
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_test() {
        let task_title = 
*/
