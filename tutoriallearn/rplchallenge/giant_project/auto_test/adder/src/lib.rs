// 正方形構造 マクロ構造を設定
#[derive(Debug)]
struct Square {
    start_x: u32,
    start_y: u32,
    end_x: u32,
    end_y: u32
}

// インターフェース側のメソッドを定義
impl Square {
    // 四角形の中に座標が含まれているかの判定メソッド
    fn can_hold(&self, pos_x: u32, pos_y: u32) -> bool {
        let is_hold_x = pos_x >= self.start_x && self.end_x >= pos_x;
        let is_hold_y =  pos_y >= self.start_y && self.end_y >= pos_y;
        is_hold_x && is_hold_y
    }

    

}

// 正方形に入っているかのテスト
#[test]
fn is_can_hold() {
    let sqare_data = Square{
        start_x: 100,
        start_y: 100,
        end_x: 150,
        end_y: 130,
    };
    assert!(sqare_data.can_hold(100, 100));
    assert!(sqare_data.can_hold(150, 130));

}

// 正方形の範囲外判定テスト
#[test]
fn is_cannot_hold() {
    let sqare_data = Square{
        start_x: 100,
        start_y: 100,
        end_x: 150,
        end_y: 130,
    };
    assert!(sqare_data.can_hold(151, 100) == false);
    assert!(sqare_data.can_hold(150, 190) == false);

}