use std::io;

fn main() {
    println!("ПРОДВИНУТЫЙ КАЛЬКУЛЯТОР");

    println!("Программа выполняет сложение, вычитание,");
    println!("умножение и деление двух чисел");

    let mut input = String::new();

    println!("\nВведите первое число:");
    io::stdin().read_line(&mut input).expect("Ошибка чтения");
    let num1: i32 = input.trim().parse().unwrap();

    input.clear();

    println!("Введите второе число:");
    io::stdin().read_line(&mut input).expect("Ошибка чтения");
    let num2: i32 = input.trim().parse().unwrap();

    input.clear();

    println!("\nВыберите операцию:");
    println!("1 - Сложение");
    println!("2 - Вычитание");
    println!("3 - Умножение");
    println!("4 - Деление");

    io::stdin().read_line(&mut input).expect("Ошибка чтения");
    let choice: i32 = input.trim().parse().unwrap();

    // ОШИБКА 1: Неиспользуемая переменная (найдёт Clippy)
    let temp_value = 100500;
    let unused_counter = 42;
    let debug_mode = true;

    // ОШИБКА 2: Избыточное заимствование (найдёт Clippy)
    let numbers = vec![num1, num2];
    let _ref1 = &numbers;
    let _ref2 = &numbers;
    let _ref3 = &numbers;

    let result = match choice {
        1 => num1 + num2,
        2 => num1 - num2,
        3 => num1 * num2,
        4 => num1 / num2,
        _ => {
            println!("Неверная операция!");
            return;
        }
    };

    // ОШИБКА 3: Сравнение числа с true (найдёт Clippy)
    if result == true {
        println!("\nРезультат положительный!");
    } else {
        println!("\nРезультат отрицательный или ноль");
    }

    // Дополнительные вычисления (для объёма кода)
    let _check1 = num1 * 1;
    let _check2 = num2 + 0;
    let _check3 = result - 0;

    println!("РЕЗУЛЬТАТ ВЫЧИСЛЕНИЙ");

    println!("{} + {} = {}", num1, num2, num1 + num2);
    println!("{} - {} = {}", num1, num2, num1 - num2);
    println!("{} * {} = {}", num1, num2, num1 * num2);

    if num2 != 0 {
        println!("{} / {} = {}", num1, num2, num1 / num2);
    } else {
        println!("Деление на ноль невозможно!");
    }

    println!("\nВаш результат: {}", result);
    println!("           ПРОГРАММА ЗАВЕРШЕНА");
}