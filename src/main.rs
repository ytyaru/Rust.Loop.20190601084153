/*
 * Rustのloop。
 * CreatedAt: 2019-06-01
 */
fn main() {
//    loop { println!("無限ループ！　端末次第だがCtrl+Cキーで終了するだろう。"); }
    let mut count = 0;
    loop {
        println!("loop num {}.", count);
        count += 1;
        if 9 < count { break; }
    }
}

