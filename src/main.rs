use std::env;

fn help() {
    println!(
        "usage:
fibonacci-in-rust <u32>
    Given a number num, calculate n-th Fibonacci number.
    Eg: fibonacci-in-rust 13"
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            let num: u32 = match args[1].parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("error: argument is not an integer (u32 expected)");
                    return;
                }
            };

            println!("{}", fib(num));
        }
        _ => {
            help();
        }
    }
}

fn fib(num: u32) -> u32 {
    match num {
        0 => return 0,
        1 => return 1,
        2 => return 1,
        _ => {
            return fib(num - 1) + fib(num - 2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn f0() {
        assert_eq!(fib(0), 0);
    }
    #[test]
    fn f1() {
        assert_eq!(fib(1), 1);
    }
    #[test]
    fn f2() {
        assert_eq!(fib(2), 1);
    }
    #[test]
    fn f14() {
        assert_eq!(fib(14), 377);
    }
    #[test]
    fn f40() {
        assert_eq!(fib(40), 102334155);
    }
}
