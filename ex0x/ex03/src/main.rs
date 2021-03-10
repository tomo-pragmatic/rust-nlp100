/// # 03.円周率
/// 
/// “Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.”という文を単語に分解し，
/// 各単語の（アルファベットの）文字数を先頭から出現順に並べたリストを作成せよ．

fn main() {
    for s in separate_words2("Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.") {
        println!("{}", s);
    }
}

/*
fn separate_words(text: &str) -> Vec<usize>{
    text.split(&[' ', ',', '.'][..]) // 要調査(宿題)
    // splitの引数は&str or char or charのスライス
    // char配列をスライスにするために[..](=std::ops::RangeFull)が必要
        .into_iter()
        .filter(|x| x != &"")
        .map(|x| x.len())
        .collect()
}
*/

fn separate_words2(text: &str) -> Vec<usize>{
    text.replace(",", "")
        .replace(".", "")
        .split_whitespace()
        .map(|x| x.len())
        .collect()
}