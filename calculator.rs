use std::io;
use std::io::Write;
use std::process;

/////////////////////    /////////////////////
//  setting up functions 
/////////////////////    /////////////////////

//check is inputted character is an operation
fn is_op(ch: char) -> bool {
    return ['+', '-', '*', '/'].contains(&ch);
}

//run the operation based on the provided operation
macro_rules! run_operation {
    ($stack:ident, $op:tt) => {{
        $stack.push(&$stack[$stack.len() - 2] $op &$stack[$stack.len() - 1]);
        $stack.remove($stack.len() - 2);
        $stack.remove($stack.len() - 2);
    }};
}

//print out error message and set error to true
fn error_occurred(err_type: &str, err_message: &str, err: &mut bool) {
    println!("{err_type} ERROR: {err_message}");
    *err = true;
}

fn main() {

    /////////////////////    /////////////////////
    //  defining variables
    /////////////////////    /////////////////////

    let mut num_build_list: Vec<char> = Vec::new();

    let mut stack: Vec<f64> = Vec::new();

    let mut negative = false;

    let mut error = false;

    /////////////////////    /////////////////////
    //  taking inputs
    /////////////////////    /////////////////////

    loop {

        let mut operation = ' ';

        print!("> ");

        //getting acutal input
        io::stdout().flush().unwrap();
        let mut input_equation = String::new();
        
        //reading and formatting input
        io::stdin().read_line(&mut input_equation).expect("error reading user input line");
        let equation = input_equation.trim();

        //turning equation into equationlist
        let mut equation_list: Vec<char> = equation.chars().collect();
        
        //add empty char at the end to deal with some weird edge cases
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

        //test out `for ind in equation_list.len() {`
        //if i do this dont forget to remove the ind+=1 at the end of this loop
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
            
            //checking for operation
            if is_op(character) && equation_list[ind+1] == ' ' {
                operation = character;
            }

            //syntax error if there is a misplaced operation
            if is_op(character) && stack.len() < 2 && negative == false {
                error_occurred("SYNTAX", "incorrect operation placement", &mut error);
                break;
            }
            
            /////////////////////    /////////////////////
            //  parse number input
            /////////////////////    /////////////////////

            //syntax error if invalid character is found
            if !character.is_digit(10) && !is_op(character) && character != ' ' && character != '.' {
                error_occurred("SYNTAX", "unrecognized character found", &mut error);
                break;
            } 

            //push numbers to num_build_list
            if character.is_digit(10) || character == '.' {
                num_build_list.push(character);
            }

            // creating and pushing number into stack
            if (character == ' ' || is_op(character)) && (!num_build_list.is_empty()) {

                //syntax error if it will be invalid from extra decimal point
                if num_build_list[num_build_list.len()-1] == '.' {
                    error_occurred("SYNTAX", "unexpected decimal point", &mut error);
                    break;
                }

                //collecting all of num_build_list and putting it into unstripped_num
                let unstripped_num: String = num_build_list.clone().into_iter().collect();

                //syntax error if the number has too many decimals
                if unstripped_num.matches('.').count() > 1 {
                    error_occurred("SYNTAX", "too many decimals", &mut error);
                    break;
                }

                //cleaning and parsing into 64bit float
                let number_into_stack: f64 = unstripped_num.parse().expect("error parsing to f64");

                //make negative (if its negative) then push to stack
                if negative == true {
                    stack.push(0.0-number_into_stack);
                    negative = false;
                } else {stack.push(number_into_stack);}

                //clear num_build_list to prepare for the next number if there is another
                num_build_list.clear();
            } 

            /////////////////////    /////////////////////
            //  perform operations
            /////////////////////    /////////////////////

            if stack.len() >= 2 {

                //match operation to actually run it 
                match operation {
                    '*' => {run_operation!(stack, *);}
                    '/' => {
                        
                        //math error if there is division by zero
                        if stack[stack.len()-1] == 0.0 {
                            error_occurred("MATH", "div/0 -> undefined", &mut error);
                            break;
                        } else {run_operation!(stack, /);}
                    }
                    '-' => {run_operation!(stack, -);}
                    '+' => {run_operation!(stack, +);}
                    _ => {}
                }
                
                operation = ' ';
            }

            //increase index of current character to parse
            //remove this line if i change to for loop
            ind += 1; 
        }

        /////////////////////    /////////////////////
        //  print final answer and reset
        /////////////////////    /////////////////////

        //check if there is more then 1 thing left in stack
        if stack.len() > 1 && !error{
            error_occurred("SYNTAX", "too few operations", &mut error);
        }

        //print out result if everything went well
        if !error && stack.len() > 0 { 
            println!("{}",stack[0]);
        }

        //reset 
        error = false;
        num_build_list.clear();
        stack.clear();
    }
}
