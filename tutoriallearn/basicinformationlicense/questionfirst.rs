fn main() {
    // バイトデータを格納する動的リストを設定
    let mut v = vec![];

    let mut index = 50;

    let mut count = 1;
    let mut result: String = "".to_string();
    result.push_str(&"10進数 入力内容".to_string());
    result.push_str(&index.to_string());
    result.push_str(&"  2進数変換結果 ".to_string());
    while count < 9 {
        println!("nishin count value {}", count);
        // 2進数変換処理
        v.push(index % 2);
        index = index / 2;
        count += 1;
    }

    count = 0;
    while count < 8 {
        // intオブジェクトに対して文字列変換メソッドにアクセスする
        result.push_str(&v[count].to_string());
        // 次のビットへシフトする
        count += 1;
    }

    // 2新数の翻訳結果を出力する
    println!("{}", result);

}
