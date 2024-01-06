const WE: &str = "WasmEdge";

fn main() {
    for i in 0..10 {
        if i < 3 {
            println!("Howdy {}!", WE);
        } else if i == 3 {
            println!("Hola {}!", WE);
        } else if i == 4 {
            println!("Bonjour {}!", WE);
        } else if i == 5 {
            println!("guten tag {}!", WE);
        } else if i == 6 {
            println!("WasmEdge 你好!");
        } else if i == 7 {
            println!("こんにちは {}!", WE);
        } else {
            println!("Salve {}!", WE)
        }
    }
}
