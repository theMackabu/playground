mod alloc;
use alloc::List;

fn main() {
    let mut list: List<&str> = List::new();

    list.push("hello");
    list.push("world");

    let list_macro = list!["meow", ":3"];
    let list_from = List::from_slice(&[1, 2, 3, 4, 5]);
    let msg = list.iter().map(|val| format!("{val} ")).collect::<String>();

    println!("msg: {}", msg.trim());
    println!("macro: {list_macro:?}");
    println!("list: {list:?}, len {}", list.len());
    println!("list_from: {list_from:?}, len {}", list_from.len());
}
