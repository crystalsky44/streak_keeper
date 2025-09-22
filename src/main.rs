use std::io;
use std::io::Write;



fn main() {
    let task_0 = run();
    
    println!("\n++Your task++");
    print!("Title: {}\nDetail: {}", task_0.task_title, task_0.task_detail);
}

fn run() -> Task {
    let task = Task::new();
    task
}
    /*
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
        input_detail.clear();

        print!("Input is empty!\nPlease input your task detail: ");
        io::stdout()
            .flush()
            .expect("just_for_test");

        // awake input
        io::stdin()
            .read_line(&mut input_detail)
            .expect("Task title is required to add task");

        input_detail = String::from(input_detail.trim());

    } 

    for x in input_detail.chars().into_iter() {
        let code = x as u32;
        println!("U+{:04x}", code);
    }

    let task_0 = Task {
     task_title: input_title,
     task_detail: input_detail,
    };

    task_0
    */

struct Task {
    task_title: String,
    task_detail: String,
}
 
impl Task {
    fn new() -> Task {
        print!("Please input your task title: ");
        let input_title = Self::input("title".to_string());

        print!("Please input your task detail: ");
        let input_detail = Self::input("detail".to_string());

        let task_0 = Task {
            task_title: input_title,
            task_detail: input_detail,
        };

        task_0
 
    }
    
    fn input(data_name: String) -> String {
        let mut input = String::new();

        while input.is_empty() {
            // flush output
            io::stdout()
                .flush()
                .expect("just_for_test");

            io::stdin()
                .read_line(&mut input)
                .expect("Task title is required to add task {data_name}");

            if input.trim().is_empty() {
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
