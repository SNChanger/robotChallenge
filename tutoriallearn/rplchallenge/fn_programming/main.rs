use std::io;

fn main() {
   println!("数値を入力してみよう。");
   println!("下の内容を自動で計算してくれるよ。");
   println!("□+△=");
   println!("□-△=");
   println!("□*△=");

   println!("キーボードで計算の最初の□を入力してね");

   let mut x = String::new();

   // 入力内容を受け取る
   io::stdin()
       .read_line(&mut x)
       .expect("数値の受け取りだめだったよ");

   let mut y = String::new();

   println!("キーボードで計算の△を入力してね");

   
   io::stdin()
       .read_line(&mut y)
       .expect("数値の受け取りだめだったよ");

    let x: usize = x
       .trim()
       .parse()
       .expect("");

    let y: usize = y
       .trim()
       .parse()
       .expect("");

    let add_result_num = add_result(x, y);
    let sub_result_num = sub_result(x, y);
    let mul_result_num = mul_result(x, y);

    println!("計算結果だよー");

    println!("{x}+{y}={add_result_num}");
    println!("{x}-{y}={sub_result_num}");
    println!("{x}*{y}={mul_result_num}");


}

fn add_result(x: usize, y: usize) -> usize {
    x + y
}

fn sub_result(x: usize, y: usize) -> usize {
   x - y
}

fn mul_result(x: usize, y: usize) -> usize {
    x * y
}