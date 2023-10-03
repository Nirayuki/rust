use std::io;

fn main() {
    let mut close: bool = false;
   while close == false {
        let mut menu = String::new();
        println!("--------------------Menu--------------------");
        println!("1- +");
        println!("2- -");
        println!("3- /");
        println!("4- *");
        println!("5 - close");
        io::stdin().read_line(&mut menu).expect("failed to readline");
    
        match &menu[..]{
            "1" => {
                somar(20, 10);
            }
            "5" => {
                close = true;
            }
        }
   }
}

fn somar(num: i32, num2: i32){
    println!("A soma ficou: {}", num + num2);
}

fn subtracao(num: i32, num2: i32){
    println!("A subtração ficou: {}", num - num2);
}

fn divisao(num: i32, num2: i32){
    println!("A divisão ficou: {}", num / num2);
}

fn multiplicacao(num: i32, num2: i32){
    println!("A multiplicação ficou: {}", num * num2);
}
