fn main() {
    println!("Hello, world!");

    // traitの話
    // next()を実装しているIteratorトレイトを実装している型Iteratorで定義されてる他の関数をよぶことができる
    // ダックタイピングのようなものかな

    // マーカトレイト
    // 印的なサムシング

    // 型のジェネリクス、Tとかのやつ

    // 所有権の話

    // ライフタイムの話、可変な参照と不変な参照の話、意味はわかるけどこんなごっちゃになることあるんかと思う

    // 旧いRustはライフタイムがレキシカルスコープだったけど、今はノンレキシカルスコープになったらしい、NLL(Non-Lexical Lifetimes)というらしい

    // RAII (Resource Acquisition Is Initialization)の話
    // dropするならデストラクタとしてDropトレイトを利用する

    // スレッドの話
    use std::thread;
    let handle = thread::spawn(|| {
        println!("Hello, world!");
    });

    dbg!(handle.join());

    let mut handles = Vec::new();

    // xの参照がスレッドが終了まで必要なので、moveをつける
    for x in 0..10 {
        handles.push(thread::spawn(move || {
            println!("Hello, world! {}", x);
        }));
    }

    for handle in handles {
        let _ = handle.join();
    }

    // 共有メモリ

    // Rc →マルチスレッドには使えない
    // Arc →マルチスレッドに使える
    // Arc自体は排他制御を行わないので、Mutexを使う
    use std::sync::{Arc, Mutex};
    let mut handles2 = Vec::new();
    let mut data = Arc::new(Mutex::new(vec![1; 10]));

    for x in 0..10 {
        let data_ref = data.clone();
        handles2.push(thread::spawn(move || {
            let mut data = data_ref.lock().unwrap();
            data[x] += 1;
        }));
    }

    for handle in handles2 {
        let _ = handle.join();
    }

    dbg!(data);
}
