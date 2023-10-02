fn main() {
    let mut number = 0;
    println!("The number is: {}", number);

    loop {
        number += 1;

        if number == 10 {
            println!("A contagem acabou!!");
            break;
        }

        println!("{}", number);
    }
}
