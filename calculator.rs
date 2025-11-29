use std::io;
use std::io::Write;
use std::process;


/////////////////////    /////////////////////
//  setting up functions
/////////////////////    /////////////////////


// check is inputted character is an operation
fn is_op(ch: char) -> bool {
    ['+', '-', '*', '/'].contains(&ch)
}

macro_rules! run_operation {
    ($stack:ident, $op:tt) => {{
        $stack.push(&$stack[$stack.len() - 2] $op &$stack[$stack.len() - 1]);
        $stack.remove($stack.len() - 2);
        $stack.remove($stack.len() - 2);
    }};
}

fn main() {

    /////////////////////    /////////////////////
    //  defining variables
    /////////////////////    /////////////////////



    let mut num_build_list = vec![' '];
    num_build_list.clear();

    let mut stack: Vec<f64> = Vec::new();

    let mut negative = false;

    let mut error = false;



    /////////////////////    /////////////////////
    //  taking inputs
    /////////////////////    /////////////////////


    loop {

        let mut operation = ' ';

        print!("> ");

        //taking acutal input

        io::stdout().flush().unwrap();
        let mut input_equation = String::new();
        
        //reading and formatting input

        io::stdin().read_line(&mut input_equation).expect("error reading line");
        let equation = input_equation.trim();

        //turning equation into equationlist

        let mut equation_list: Vec<char> = equation.chars().collect();

        equation_list.push(' ');



        /////////////////////    /////////////////////
        //  quit process if needed
        /////////////////////    /////////////////////


        if equation.to_lowercase() == "quit" || equation.to_lowercase() == "q" {
            process::exit(0);
        }


        /////////////////////    /////////////////////
        //  looping through all characters to parse input
        /////////////////////    /////////////////////

        
        let mut ind = 0;

        while ind < equation_list.len() {

            let character = equation_list[ind];


            /////////////////////    /////////////////////
            //  checking if negative then checking for operation
            /////////////////////    /////////////////////


            //checking if negative

            if ind < equation_list.len() {
                if character == '-' && equation_list[ind+1].is_digit(10) && num_build_list.is_empty() {
                    negative = true;
                }
            }

            
            // checking for operation

            if is_op(character) && equation_list[ind+1] == ' ' {
                operation = character;
            }

            if is_op(character) && stack.len() < 2 && negative == false {

                error = true;
                println!("SYNTAX ERROR: operation found too early");
            }
            

            /////////////////////    /////////////////////
            //  parse number input
            /////////////////////    /////////////////////


            //syntax error if invalid character is found

            if !character.is_digit(10) && !is_op(character) && character != ' ' && character != '.' {
                println!("SYNTAX ERROR: unrecognized character found");
                error = true;
                break;
            } 

            //push numbers to num_build_list

            if character.is_digit(10) || character == '.' {
                num_build_list.push(character);
            }


            // creating and pushing a non-negative number into stack

            if (character == ' ' || is_op(character)) && (!num_build_list.is_empty()) {

                // collecting all of num_build_list and putting it into unstripped_num
                let unstripped_num: String = num_build_list.clone().into_iter().collect();

                //cleaning and parsing into 32bit float
                let number_into_stack: f64 = unstripped_num.parse().expect("error parsing to f64");

                //make negative (if its negative) then push to stack

                if negative == true {
                    stack.push(0.0-number_into_stack);
                    negative = false;
                } else {stack.push(number_into_stack);}

                
                num_build_list.clear();
            } 



            /////////////////////    /////////////////////
            //  perform operations
            /////////////////////    /////////////////////


            if stack.len() >= 2 {

                if operation == '*' {
                    run_operation!(stack, *);
                }
                if operation == '/' {
                    run_operation!(stack, /);
                }
                if operation == '-' {
                    run_operation!(stack, -);
                }
                if operation == '+' {
                    run_operation!(stack, +);
                }
                operation = ' ';
            }

            ind += 1;
        }


        /////////////////////    /////////////////////
        //  print final answer and reset
        /////////////////////    /////////////////////


        if !error && stack.len() > 0 { 
            println!("{}",stack[0]);
            stack.clear();
        //reset error
        } else if error == true {
            error = false;
            stack.clear();
        }
    }
}
