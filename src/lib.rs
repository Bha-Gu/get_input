use std::{io,str::FromStr};
//Take string input
pub fn get_string(print: &str) -> String {
    let mut string = String::new();
    println!("{}", print);
    io::stdin()
        .read_line(&mut string)
        .expect("Failed to read line");
    string
}

fn get_string_trim(input: &str) -> String{
	get_string(input).trim().to_string()
}

const INVALID_INPUT: &str= "Not a valid input\nTry again";
const INPUT_OUT_OF_RANGE: &str = "Input out of range\nTry again";

pub fn get_num<T>(print: &str) -> T
    where T: FromStr 
{
    loop {
        let u: T = match get_string_trim(print).parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", &INVALID_INPUT);
                continue;
            }            
        };
        return u;
    }
}

pub fn get_num_bw<T>(low: T, up: T, print: &str) -> T 
	where T: FromStr + PartialOrd 
{
    loop {
        let u = get_num(print);
        if low <= u && u <= up {
            return u;
        } else {
            println!("{}", &INPUT_OUT_OF_RANGE);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_unsigned() {
        let ui8: u8 = get_num_bw(1,8,"Enter a number between 1 & 8");
        let ui16: u16 = get_num_bw(1,8,"Enter a number between 1 & 8");
        let ui32: u32 = get_num_bw(1,8,"Enter a number between 1 & 8");
        let ui64: u64 = get_num_bw(1,8,"Enter a number between 1 & 8");
        let ui128: u128 = get_num_bw(1,8,"Enter a number between 1 & 8");
        let uisize: usize = get_num_bw(1,8,"Enter a number between 1 & 8");
    }

    #[test]
    fn it_works_signed() {
        let si8: i8 = get_num_bw(-1,8,"Enter a number between 1 & 8");
        let si16: i16 = get_num_bw(-1,8,"Enter a number between 1 & 8");
        let si32: i32 = get_num_bw(-1,8,"Enter a number between 1 & 8");
        let si64: i64 = get_num_bw(-1,8,"Enter a number between 1 & 8");
        let si128: i128 = get_num_bw(-1,8,"Enter a number between 1 & 8");
        let sisize: isize = get_num_bw(-1,8,"Enter a number between 1 & 8");
    }
}
