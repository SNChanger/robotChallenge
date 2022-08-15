
use std::collections::HashMap;

fn main() {

    // 単方向リストの整形サンプル
    display_vector_data();

    // stringデータを単一文字で抽出や利用しているぶぶん
    display_string_to_chars();

    // 速度をキーにした辞書データを作成
    let hash_result_message = display_hashmap(30);

    println!("now speed message {}" , hash_result_message);

}

//　翻訳元 https://doc.rust-lang.org/book/ch08-01-vectors.html
fn display_vector_data() {
    let mut speed_vecotrs = vec![];

    // 単方向リストでのデータ追加
    speed_vecotrs.push(0);
    speed_vecotrs.push(10);
    speed_vecotrs.push(15);
    speed_vecotrs.push(30);

    let speed_data = &speed_vecotrs[0];


    println!("speed vector first data {}", speed_data);

    for vector_data in &speed_vecotrs {
        println!("speed data : {} km", vector_data);
    }

}

fn display_string_to_chars() {
    // C言語 → rust へのDX妄想
    let message = "こんにちは、にほん";
    let mut count = 0;
    for charcter in message.chars() {
        count += 1;
        // stringを分解して文字データ単位で表示する
        println!("{}.{}", count, charcter);
    }

}


fn display_hashmap(speed: i32) -> String {

    let mut speed_messages = HashMap::new();
    speed_messages.insert(0, String::from("stop"));
    speed_messages.insert(10, String::from("start_speed"));
    speed_messages.insert(15, String::from("low_speed"));
    speed_messages.insert(30, String::from("normal_speed"));

    speed_messages.get(&speed).unwrap().to_string()
}