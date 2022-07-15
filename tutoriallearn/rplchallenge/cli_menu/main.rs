use std::io;

fn main() {


    let menu_titles = ["くるま", "冷蔵庫", "バイク", "テレビ", "改札機"];

    println!("メニューを選んでね");
    let mut menu_count = 0;
    while menu_count < menu_titles.len() {        
        let menu_name = menu_titles[menu_count];
        println!("{menu_count}. {menu_name}");
        menu_count += 1;
    }

    let mut select_index = String::new();
    
    io::stdin()
        .read_line(&mut select_index)
        .expect("キーボードの入力失敗しました");

    let select_index: usize = select_index
        .trim()
        .parse()
        .expect("メニューは番号で入力してください");

    if select_index >= menu_titles.len() {
        let menu_min_len = 0;
        let menu_len = menu_titles.len()-1;
        println!("メニューは{menu_min_len}から{menu_len}までです");
        return
    }

    let menu_descriptions = ["4つのタイヤで移動する箱", "四角い箱で食べ物を冷やせる", "2つのタイヤで1人でのる乗り物", "電源を付けるとアニメとかが見れるはこ", "キップとかSuicaで通り抜けられる"];

    let menu_name = menu_titles[select_index];
    let menu_description = menu_descriptions[select_index];

    println!("{menu_name}");
    println!("{menu_description}");

}