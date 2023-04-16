use std::io;
fn main() {
    let mut x = String::new();
    let mut y = String::new();

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

    let result:f64 = x + y;

    println!("Сумма двух чисел равна! {}", result)
}