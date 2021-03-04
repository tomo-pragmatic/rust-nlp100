/// # 00.文字列の逆順 
/// 
/// 文字列”stressed”の文字を逆に（末尾から先頭に向かって）並べた文字列を得よ．

fn main() {
    println!("{}", reverse("stressed"));
}

fn reverse(text: &str) -> String {
    text.chars().rev().collect::<String>()
}

#[test]
fn test_reverse() {
    assert_eq!(reverse("stressed"), "desserts");
    assert_eq!(reverse("ちいさいか"), "かいさいち");
    assert_eq!(reverse("春眠不覚暁"), "暁覚不眠春");
}