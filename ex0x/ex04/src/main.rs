/// # 04.元素記号
/// 
/// “Hi He Lied Because Boron Could Not Oxidize Fluorine. 
/// New Nations Might Also Sign Peace Security Clause. Arthur King Can.”という文を
/// 単語に分解し，1, 5, 6, 7, 8, 9, 15, 16, 19番目の単語は先頭の1文字，
/// それ以外の単語は先頭の2文字を取り出し，取り出した文字列から単語の位置（先頭から何番目の単語か）への
/// 連想配列（辞書型もしくはマップ型）を作成せよ．

use std::collections::HashMap;

fn main() {
    let map = create_atomic_symbols();

    for k in map.keys() {
        println!("{}, {}", k, map[k])
    }
}

fn create_atomic_symbols() -> HashMap<String, usize> {
    const PHRASE: &str  = "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can.";
    const SINGLE_INDECES: [usize; 9] = [1, 5, 6, 7, 8, 9, 15, 16, 19];
    
    let mut map = HashMap::new();
    for (i, word) in PHRASE.replace(".", "").replace(",", "").split(&[' ', '.', ','][..]).enumerate() {
        map.insert(word.get(..if SINGLE_INDECES.contains(&(i+1)) {1} else {2}).unwrap().to_string(), i + 1);
    }
    map
}
