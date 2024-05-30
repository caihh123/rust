use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let rand_guess = rand::thread_rng().gen_range(1..101);
    loop {
        let mut guess = String::new();
        println!("请输入你要猜的数字:");
        match io::stdin().read_line(&mut guess) {
            Ok(_) => {
                // println!("{guess}");
            }
            Err(error) => println!("error: {error}"),
        }
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("您的输入有错误,");
                continue;
            }
        };
        match guess.cmp(&rand_guess) {
            Ordering::Equal => {
                println!("恭喜你，猜对了");
                break;
            }
            Ordering::Less=> {
                println!("你猜的数字太小了");
            }
            Ordering::Greater=>{
                println!("你猜的数字太大了");
            }
        }
    }
}
