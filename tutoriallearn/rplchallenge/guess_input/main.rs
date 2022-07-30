use std::io;

fn main() {
   println!("数値を入力してみよう");

   println!("キーボードで文字を入力してね");

   let mut guess = String::new();

   // 入力内容を受け取る
   io::stdin()
       .read_line(&mut guess)
       .expect("faile to read line");

    
    println!("入力した内容はこれだよ: {guess}");

}