fn main() {

    // 平均値を宣言
    let average_value = 50;

    // 標準偏差
    let standard_deviation = 10;
    println!("平均値 入力内容 {}", average_value);
    println!("標準偏差 {}", standard_deviation);


    // 標準偏差の差分を出力
    println!("標準偏差下限 {}", min_standard_deviation(average_value, standard_deviation).to_string());
    println!("標準偏差上限 {}", max_standard_deviation(average_value, standard_deviation).to_string());

}

// 標準偏差の下限を計算
fn min_standard_deviation(average_value: i32, standard_deviation: i32) -> i32 {
    average_value - standard_deviation
}

// 標準偏差の上限を計算
fn max_standard_deviation(average_value: i32, standard_deviation: i32) -> i32 {
    average_value + standard_deviation
}
