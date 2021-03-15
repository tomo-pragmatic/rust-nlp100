/// # 09.Typoglycemia
/// 
/// スペースで区切られた単語列に対して，各単語の先頭と末尾の文字は残し，
/// それ以外の文字の順序をランダムに並び替えるプログラムを作成せよ．
/// ただし，長さが4以下の単語は並び替えないこととする．
/// 適当な英語の文（例えば"I couldn't believe that I could actually understand 
/// what I was reading : the phenomenal power of the human mind."）を与え，その実行結果を確認せよ．

use regex::{Regex, Captures};
use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    const PHRASE: &str = "I couldn't believe that I could actually understand what I was reading : the phenomenal power of the human mind.";
    println!("{}", typoglycemia(PHRASE));
}

fn typoglycemia(phrase: &str) -> String {
    let re = Regex::new(r"[a-zA-Z]+").unwrap();
    re.replace_all(phrase, |caps: &Captures| swap(&caps[0]))
      .to_string()
}

fn swap(word: &str) -> String {
    if word.len() < 4 {
        String::from(word)
    } else {
        let mut v: Vec<char> = word.chars().collect();
        let n = v.len();
        v[1..n-1].shuffle(&mut thread_rng());
        v.into_iter().collect()
    }
}

