mod alloc;
mod value;

use alloc::List;
use value::Value;

fn main() {
    let mut list: List<&str> = List::new();
    let mut list_dyn: List<Value> = List::new();

    list.push("hello");
    list.push("world");

    list_dyn.push(true.into());
    list_dyn.push(list.to_owned().into());
    list_dyn.push(Value::Slice(&[Value::Number(1), Value::Number(2)]));

    list.len;

    let list_macro = list!["meow", ":3"];
    let list_from = List::from_slice(&[1, 2, 3, 4, 5]);
    let msg = list.iter().map(|val| format!("{val} ")).collect::<String>();

    for val in list_dyn.iter() {
        println!("{val}");
    }

    println!("msg: {}", msg.trim());
    println!("macro: {list_macro:?}");
    println!("list: {list:?}, len {}", list.len());
    println!("list_dyn: {list_dyn:?}, len {}", list_dyn.len());
    println!("list_from: {list_from:?}, len {}", list_from.len());
}

