// main.rs
fn suma(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let num1 = 5;
    let num2 = 7;
    let resultado = suma(num1, num2);
    println!("La suma de {} y {} es: {}", num1, num2, resultado);
}