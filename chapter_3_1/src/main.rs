// fn print(s: [u8]) {
//     println!("{:?}", s);
// }

fn print2(s: Box<[u8]>) {
    println!("{:?}", s);
}

// 本には載ってないけど...byte code -> 文字列に変換する方法
fn print_decoded(s: Box<[u8]>) {
    println!("{:?}", String::from_utf8_lossy(&s));
}

fn main() {
    // Boxの使い方, p48
    // b'x'とかするとバイトコードになる、つまり数字ってわけね、(u8) 
    let byte_array = [b'h', b'e', b'l', b'l', b'o'];
    // print(byte_array);
    print2(Box::new(byte_array));
    print_decoded(Box::new(byte_array));

    // ----代替知ってることが続く
    // p62
    // fmtのプレースホルダ、{}, {:?},など

    // マクロの話、色々あるっぽい、必要に応じて調べるかね
    // unimplemented!やtodo!のような実装補助系のマクロもあるらしい、イケイケだね
    // unreachable!も欲しいやつだ


    // p-68
    // トレイトの話
    // Copy, Cloneの関係
    // Eq, PartialEqの関係
    // PartialEq, PartialOrdの関係
    // float系はNaNがあるので、Eq, Ordは実装できない(Partial系は反射律を満たさないってことね
    
}
