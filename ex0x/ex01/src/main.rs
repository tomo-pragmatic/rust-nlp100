/// # 01.「パタトクカシーー」 
/// 
/// 「パタトクカシーー」という文字列の1,3,5,7文字目を取り出して連結した文字列を得よ．

fn main() {
    println!("{}", pick_odd("パタトクカシーー"));
}

fn pick_odd(text: &str) -> String {
    text.chars()
        .step_by(2)
        .collect()
}

#[test]
fn test_pick_odd() {
    assert_eq!(pick_odd("パタトクカシーー"), "パトカー");
    assert_eq!(pick_odd("fsoaunr"), "four");
}