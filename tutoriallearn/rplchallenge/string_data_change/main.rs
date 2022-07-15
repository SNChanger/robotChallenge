use std::io;

fn main() {
   println!("苗字を入力してね。");


   let mut first_name = String::new();

   // 入力内容を受け取る
   io::stdin()
       .read_line(&mut first_name)
       .expect("");

   let mut last_name = String::new();

   println!("名前を入力してね。");

   
   io::stdin()
       .read_line(&mut last_name)
       .expect("");

    println!("以下の名前で情報局に登録するよ");

    let full_name = create_fullname(first_name, last_name);

    println!("{}", full_name);

}

fn create_fullname(first_name: String, _last_name: String) -> String {
    let mut full_name = String::from(first_name.trim());
    full_name.push_str(" ");
    full_name.push_str((&_last_name).trim());
    full_name
}
