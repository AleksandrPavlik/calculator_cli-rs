use std::io;
fn main() {
    let mut x = String::new();
    let mut y = String::new();
    let mut choose = String::new();

    println!("Введите первое число.");

    io::stdin().read_line(&mut x)
    .expect("не получилось считать");

    let x:f64 = match x.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Введен символ, а не число"),
    };

    println!("Введите второе число.");

    io::stdin().read_line(&mut y)
    .expect("не получилось считать");

    let y:f64 = match y.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Введен символ, а не число"),
    };

    println!("Что будем делать?
    1. +
    2. -
    3. /
    4. *");

    io::stdin().read_line(&mut choose)
    .expect("не получилось считать");

    let choose:i8 = match choose.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("не получилось спарсить, возможно вы ввели символ")
    };


    match choose {
        1 => println!("Сумма двух чисел равна. {}", x + y),
        2 => println!("Сумма двух чисел равна. {}", x - y),
        3 => if y <= 0.0 {
            panic!("Нельзя делить на ноль или меньшую сумму.");
        } else {
            println!("Сумма двух чисел равна. {}", x / y);
        }
        4 => println!("Сумма двух чисел равна. {}", x * y),
        _=> panic!("Empty"),
    };
}