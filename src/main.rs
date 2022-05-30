use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("請猜一個數字!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("請輸入一個數字。");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("讀取該行失敗");

        // let guess: u32 = guess.trim().parse().expect("請輸入一個數字! ");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("你猜的數字: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => print!("太小了! "),
            Ordering::Greater => print!("太大了! "),
            Ordering::Equal => {
                print!("獲勝 ");
                break;
            }
        }
    }
}
