use std::io::{self,Write};

fn main() {
    println!("═══════════════════════════════");
    println!("КАЛЬКУЛЯТОР С ПРОМЕЖУТОЧНЫМИ ВЫЧИСЛЕНИЯМИ");
    println!("═══════════════════════════════");
    println!("Команды:");
    println!("  <число>              - ввести число");
    println!("  + <число>            - сложить");
    println!("  - <число>            - вычесть");
    println!("  * <число>            - умножить");
    println!("  / <число>            - разделить");
    println!("  c                    - очистить память");
    println!("  exit                 - выйти");
    println!("\nТекущее значение: 0\n");

    let mut memory: f64 = 0.0;
    let count = 0; 
    while true {
        print!("[{}] > ", memory);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() == true{
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
