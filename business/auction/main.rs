use std::{thread, time};

#[derive(Debug)]
struct SaleItem {
    ManagementNumber: u64,
    Name: String,
    StartPrice: u64
}


fn main() {

    println!("勇者の戦利品オークションを開始します!");
    let wait_millis = time::Duration::from_millis(1300);

    thread::sleep(wait_millis);

    println!("オークションの説明を開始いたします");

    thread::sleep(wait_millis);

    println!("戦利品の対象を紹介いたします");

    println!("最低値から、皆さんに値段を手を挙げていただきます");
    thread::sleep(wait_millis);
    thread::sleep(wait_millis);

    println!("");
    println!("");
    println!("");

    println!("3分以内に他の方が新しい金額を言わなければ、落札となります");

    println!("落札いただきましたら、戦利品をお持ち帰りくださいませ");

    let mut saleItems: Vec<SaleItem> = Vec::new();
    let mut sale_first = SaleItem{ManagementNumber: 100, Name: String::from("覚醒の石"), StartPrice: 1200};
    saleItems.push(sale_first);
    let mut sale_second = SaleItem{ManagementNumber: 101, Name: String::from("真祖の王冠"), StartPrice: 1200};
    saleItems.push(sale_second);

    let mut player_money = 1500;
    let mut sale_complete = false;
    let mut sale_exec_name = String::from("");
    let mut sale_exec_value = 0;
    

    println!("");
    println!("戦利品読み上げを開始いたします");
    for saleItem in saleItems {
        let management_number = &saleItem.ManagementNumber;
        let sale_name = &saleItem.Name;
        let start_price = &saleItem.StartPrice;
        println!(" 管理番号 : {management_number} 売却名義  {sale_name} 最低年収開始額 {start_price}");
        if player_money < *start_price {
            let result_value = start_price - player_money;
            println!("{sale_name}に入札するには、後{result_value}万円必要です。");
        } else {
            sale_exec_name = String::from(sale_name);
            sale_complete = true;
            sale_exec_value = saleItem.StartPrice;
            println!("{sale_name}に入札可能です");
            break;
        }
    }

    if sale_complete {
        // 入札先の登録ができてから、タイミングがあえばオーナーから連絡が来る
        println!("{sale_exec_name}に入札をいたしました。結果は後日お待ちください");
        let value_result = player_money - sale_exec_value;
        println!("オーナーの残り予算は {value_result}です。　資金がたまりましたら、またお越しください。");

    }

}


