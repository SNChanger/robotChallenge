use rand::Rng;

fn main() {
    println!("入力情報");

    let mut dice_total_num = 0;

    let mut count = 0;
    let dice_max_test_num = 4;

    while count < dice_max_test_num {
        let dice_number = rand::thread_rng().gen_range(1..=6);

        println!("サイコロ実行結果 {dice_number}");

        dice_total_num += dice_number;
        count += 1;
    }

    println!(" サイコロ実行回数  {count} 合計 {dice_total_num} ");
}
