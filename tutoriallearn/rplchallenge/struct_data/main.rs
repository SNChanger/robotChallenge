
use std::fmt;

struct Color(i32, i32, i32);


struct FassionParts {
    color: Color    
}

impl fmt::Display for FassionParts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fassioninfos!")
    }
}


struct Account {
    nickname: String,
    level: String,
    vrsnstime: u64,
    shirt: FassionParts,
}


fn main() {

   let string_str = "50";
   let profile_name = "りらっくん";
   let bear = Account {
        nickname: (&profile_name).to_string(),
        level: (&string_str).to_string(),
        vrsnstime: 100,
        shirt: FassionParts{color: Color(0,0,0)},
   };


   // プロフィール情報を表示
   println!("ワールドでの設定だよ");
   println!("{}", build_profilemessage(bear));
   println!("キャラクター属性 ");
   println!("デバッグ用パラメータ");

}

fn build_profilemessage(profile: Account) -> String {
    let message = format!("名前: {0} レベル: {1} ", profile.nickname, profile.level);
    message
}
