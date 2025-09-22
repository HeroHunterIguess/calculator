
use std::io;
use std::io::Write;
use std::process;

fn main() {

    /////////////////////    /////////////////////
    //  defining variables
    /////////////////////    /////////////////////

    
    let num_list = vec!["0","1","2","3","4","5","6","7","8","9"];
    let op_list = vec!["+","-","*","/"];


    let mut num_build_list = vec![' '];
    num_build_list.clear();

    let mut stack: Vec<f32> = Vec::new();

    let mut operation = String::new();


    /////////////////////    /////////////////////
    //  taking inputs
    /////////////////////    /////////////////////

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut error = false;

        let mut input_equasion = String::new();

        io::stdin().read_line(&mut input_equasion).expect("error reading line");

        let equasion = input_equasion.trim();

        let equasion_list: Vec<char> = equasion.chars().collect();



        /////////////////////    /////////////////////
        //  quit process if needed
        /////////////////////    /////////////////////


        if equasion.to_lowercase() == "quit" || equasion.to_lowercase() == "q" {
            process::exit(0);
        }


        /////////////////////    /////////////////////
        //  looping through all characters to parse input
        /////////////////////    /////////////////////


        
        let mut ind = 0;

        while ind < equasion_list.len() {

            let character = equasion_list[ind];

            /////////////////////    /////////////////////
            //  get operation
            /////////////////////    /////////////////////


            if op_list.contains(&character.to_string().as_str()) && stack.len() >= 2 {

                operation = character.to_string();
            } 
            if op_list.contains(&character.to_string().as_str()) && stack.len() < 2 {

                error = true;
                println!("SYNTAX ERROR: operation found too early");
            }
            

            /////////////////////    /////////////////////
            //  parse number input
            /////////////////////    /////////////////////


            if !num_list.contains(&character.to_string().as_str()) && !op_list.contains(&character.to_string().as_str()) && character != ' ' {
                println!("SYNTAX ERROR: unrecognized character found");
                error = true;
                break;
            } 

            if num_list.contains(&character.to_string().as_str()) {
                num_build_list.push(character);
            }

            if character == ' ' && !num_build_list.is_empty() || op_list.contains(&character.to_string().as_str()) && !num_build_list.is_empty() {
                let unstripped_num: String = num_build_list.clone().into_iter().collect();
                let number_into_stack: f32 = unstripped_num.parse().expect("error parsing to f32");
                stack.push(number_into_stack);
                num_build_list.clear();
            }


            /////////////////////    /////////////////////
            //  perform operations
            /////////////////////    /////////////////////


            if stack.len() >= 2 {
                if operation == "*" {
                    stack.push(&stack[stack.len()-1] * &stack[stack.len()-2]);

                    stack.remove(stack.len()-2);
                    stack.remove(stack.len()-2);

                }
                if operation == "/" {
                    stack.push(&stack[stack.len()-1] / &stack[stack.len()-2]);

                    stack.remove(stack.len()-2);
                    stack.remove(stack.len()-2);

                }
                if operation == "-" {
                    stack.push(&stack[stack.len()-1] - &stack[stack.len()-2]);

                    stack.remove(stack.len()-2);
                    stack.remove(stack.len()-2);

                }
                if operation == "+" {
                    stack.push(&stack[stack.len()-1] + &stack[stack.len()-2]);

                    stack.remove(stack.len()-2);
                    stack.remove(stack.len()-2);

                }
                operation.clear();
            }

            ind += 1;
        }


        /////////////////////    /////////////////////
        //  print final answer and reset
        /////////////////////    /////////////////////

        if error == false && stack.len() > 0 { 
            println!("{}",stack[0]);
            stack.clear();
        }
    }
}
