use std::io;
use std::io::Write;



fn main() {
    let task_0 = run();
    
    println!("\n++Your task++");
    print!("Title: {}Detail: {}", task_0.task_title, task_0.task_detail);
}

fn run() -> Task {
    let mut input_title = String::new();
    let mut input_detail = String::new();
    
    print!("Please input your task: ");
    io::stdout()
        .flush()
        .expect("just_for_test");
    // awake input
    io::stdin()
        .read_line(&mut input_title)
        .expect("Task title is required to add task");

    // until there is an input
    while input_title.trim().is_empty() {
        print!("Input is empty!\nPlease input your task: ");
        io::stdout()
            .flush()
            .expect("just_for_test");

        // awake input
        io::stdin()
            .read_line(&mut input_title)
            .expect("Task title is required to add task");
    } 

    // writing down the same code just to not think about it for now

    print!("Please input your task detail: ");
        io::stdout()
            .flush()
            .expect("just_for_test");
    // awake input
    io::stdin()
        .read_line(&mut input_detail)
        .expect("Task title is required to add task");

    // until there is an input
    while input_detail.trim().is_empty() {
        print!("Input is empty!\nPlease input your task: ");
        io::stdout()
            .flush()
            .expect("just_for_test");

        // awake input
        io::stdin()
            .read_line(&mut input_detail)
            .expect("Task title is required to add task");
    } 

    let task_0 = Task {
     task_title: input_title,
     task_detail: input_detail,
    };

    task_0

}

struct Task {
    task_title: String,
    task_detail: String,
}
 
/*
impl Task {
    fn create(title: &String) -> Task {

}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_test() {
        let task_title = String::from("\n");

        assert_eq!(task_title.trim(), "");
    }
}
