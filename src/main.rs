fn main() {

    let mut a: i32;
    let mut b: i32;
    let x: i32;
    let y: i32;

    a = 18;
    b = 60;
    x = a;
    y = b;

    while b != 0
    {
        let temp: i32 = b;
        b = a % b;
        a = temp;
    }
    println!("O MDC de {} e {} é {}", x, y, a);
}
