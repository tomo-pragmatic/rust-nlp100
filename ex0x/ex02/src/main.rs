/// # 02.「パトカー」＋「タクシー」＝「パタトクカシーー」 
/// 
/// 「パトカー」＋「タクシー」の文字を先頭から交互に連結して文字列「パタトクカシーー」を得よ．

fn main() {
    println!("{}", alternately_concat("パトカー", "タクシー"));
}

fn alternately_concat(text1: &str, text2: &str) -> String{
    let mut concatenated: String = String::new();

    if text1.chars().count() != text2.chars().count() &&
       text1.chars().count() != text2.chars().count() + 1 {
           return concatenated;
    }
    
    for i in 0..(text1.chars().count() + text2.chars().count()) {
        if i % 2 == 0 {
            concatenated.push(text1.chars().nth(i/2).unwrap());
        } else {
            concatenated.push(text2.chars().nth((i-1)/2).unwrap());
        }
    }
    concatenated
}

#[test]
fn alternately_concat_works() {
    assert_eq!(alternately_concat("パトカー", "タクシー"), "パタトクカシーー");
    assert_eq!(alternately_concat("ace", "bdf"), "abcdef");
}