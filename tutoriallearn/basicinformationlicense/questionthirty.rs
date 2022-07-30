

const PERCENT_RATE: f32 = 100.0;

fn main() {

    // ファイルサイズの転送量
    let average_file_size = 1.0;

    // ファイルの転送間隔(〇秒に一回転送される)
    let file_send_time_sec: f32 = 12.0;

    // 制御情報
    let control_header_info_rate_value = 1.2;

    // 単位変換値
    let _sec_value = 8.0;

    let mut buf = String::new();
    println!("ネット回線チェッカー");

    let ml_result = calc_sec_volume(average_file_size, control_header_info_rate_value, file_send_time_sec);
    println!("{}", format!("{}", format_args!("１秒間当たりのファイル転送速度 {}", ml_result)));

    let sync_speed_unit_result = sync_speed_unit(file_send_time_sec, _sec_value); 
    println!("{}", format!("{}", format_args!("{}Mバイト/秒", sync_speed_unit_result)));

    let line_utilization_rate_value = calc_line_utilization_rate(ml_result, sync_speed_unit_result);
    println!("{}", format!("{}", format_args!("回線利用率 {}%", line_utilization_rate_value)));

}

fn calc_sec_volume(average_file_size: f32, control_header_info_rate_value: f32, file_send_time_sec: f32) -> f32 {
    let total_volume = average_file_size * control_header_info_rate_value;
    total_volume / file_send_time_sec
}

fn sync_speed_unit(average_file_size: f32, unit_value: f32) -> f32 {
    average_file_size / unit_value
}

fn calc_line_utilization_rate(ml_result: f32,  line_volume: f32) -> f32 {
    (ml_result / line_volume) * PERCENT_RATE
}