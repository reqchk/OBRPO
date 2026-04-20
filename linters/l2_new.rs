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
    let _count = 0; 
    loop {
        print!("[{}] > ", memory);
        io::stdout().flush().expect("Ошибка вывода");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Ошибка ввода");
        let input = input.trim();

        if input.is_empty() {
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
        
        let first = match input.chars().next() {
            Some(ch) => ch,
            None => {
                println!("Ошибка: пустая строка");
                continue;
            }
        };

        let num = &input[1..].trim();

        match first {
            '+' => {
                let n: f64 = match num.parse() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("{} не является числом", num);
                        continue;
                    }
                };
                memory += n;
                println!("= {}", memory);
            }
            '-' => {
                let n: f64 = match num.parse() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("{} не является числом", num);
                        continue;
                    }
                };
                memory -= n;
                println!("= {}", memory);
            }
            '*' => {
                let n: f64 = match num.parse() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("{} не является числом", num);
                        continue;
                    }
                };
                memory *= n;
                println!("= {}", memory);
            }
            '/' => {
                let n: f64 = match num.parse() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("{} не является числом", num);
                        continue;
                    }
                };
                if n == 0.0 {
                    println!("На ноль делить нельзя");
                } else {
                    memory /= n;
                    println!("= {}", memory);
                }
            }
            _ => {
                // Если ввели просто число
                let n: f64 = match num.parse() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("{} не является числом", num);
                        continue;
                    }
                };
                memory = n;
                println!("= {}", memory);
                }
            }
        
    }
}