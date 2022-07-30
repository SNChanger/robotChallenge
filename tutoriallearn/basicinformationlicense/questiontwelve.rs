fn main() {

    // CPUのヘルツ数
    let hertz_number = 1000000000;

    let  ml_clocks_number = 0.8;

    println!("Clock Simulator(クロックシミュレータ)");

    // CPUヘルツ情報を出力
    println!("CPUヘルツ数(hertz_number) {}", hertz_number.to_string());
    println!("機械語の命令当たりのクロック数(ml_clocks_number) {}", ml_clocks_number.to_string());
    println!("1秒間実行可能命令数(sec ml exec num) {}", calc_ml_exec_num(hertz_number, ml_clocks_number).to_string());
}

fn calc_ml_exec_num(hertz_number: i32, ml_clocks_number: f32) -> f32 {
    (hertz_number as f32) / ml_clocks_number
}