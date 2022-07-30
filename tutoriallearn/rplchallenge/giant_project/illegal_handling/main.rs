use std::fs::File;
use std::io::ErrorKind;


fn main() {
    
    let master_file_name = "master_data.bin";
    let master_file = File::open(master_file_name);


    let _master_file = match master_file {
        Ok(master_file) => master_file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(master_file_name) {
                Ok(fc) => fc,
                Err(error) => panic!("新規作成と開くの両方がシステム検閲で引っ掛かりました {:?}", error),
            },
            other_error => {
                panic!("マスターデータが破損しているようです。　上書き等で修正をしてください {:?}", other_error)
            }
        }
    };


    let level = 130;
    if level < 1 || level > 100 {
        println!("キャラクターの初期設定が想定外のため、起動を終了します");
        return
    }
}