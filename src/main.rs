mod stack;
use stack::Stack;

fn main() {
    let mut a = Stack::new();
    println!("{a:?}");
    a.push(8);
    println!("{a:?}");
    a.push(16);
    println!("{a:?}");
    a.push(32);
    println!("{a:?}");
    let b = a.peek();
    let c = a.pop();
    println!("{a:?}");
    println!("{b:?} {c:?}");
}
