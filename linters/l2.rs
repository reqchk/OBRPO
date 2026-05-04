use std::io::{self,Write};

fn main() {
    println!("\nКАЛЬКУЛЯТОР С ПРОМЕЖУТОЧНЫМИ ВЫЧИСЛЕНИЯМИ\n");
    println!("Команды:");
    println!("<число>              - ввести число");
    println!("+ <число>            - сложить");
    println!("- <число>            - вычесть");
    println!("* <число>            - умножить");
    println!("/ <число>            - разделить");
    println!("c                    - очистить");
    println!("exit                 - выйти");
    println!("\nТекущее значение: 0\n");

    let mut memory = 0.0;
    let count = 0; //неиницилизованная переменная
    while true { //для rust лучше loop 
        print!("[{}] > ", memory);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() == true{ //избыточность
            continue;
        }
        
        if input == "exit" {
            break;
        }

        if input == "c" {
            memory = 0.0;
            println!("Очищено");
            continue;
        }
        
        
        let first = input.chars().next().unwrap();
        let num = &input[1..].trim();

        match first {
            '+' => {
                let n: f64 = num.parse().unwrap();
                memory += n;
                println!("= {}", memory);
            }
            '-' => {
                let n: f64 = num.parse().unwrap();
                memory -= n;
                println!("= {}", memory);
            }
            '*' => {
                let n: f64 = num.parse().unwrap();
                memory *= n;
                println!("= {}", memory);
            }
            '/' => {
                let n: f64 = num.parse().unwrap();
                if n == 0.0 {
                    println!("На ноль делить нельзя");
                } else {
                    memory /= n;
                    println!("= {}", memory);
                }
            }
            _ => {
                let n: f64 = input.parse().unwrap();
                memory = n;
                println!("= {}", memory);
                }
            }
    }
}
