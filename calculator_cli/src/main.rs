use std::io;
fn main() {
    let mut x = String::new();
    let mut y = String::new();

    println!("Введите первое число.");

    io::stdin().read_line(&mut x)
    .expect("не получилось считать");

    let x:i64 = match x.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("panic!"),
    };

    println!("Введите второе число.");

    io::stdin().read_line(&mut y)
    .expect("не получилось считать");

    let y:i64 = match y.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("panic"),
    };

    let result = x + y;

    println!("Сумма двух чисел равна! {}", result)
}