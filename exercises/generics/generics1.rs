// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.



fn main() {
    fn add<T>(list:& mut Vec<T>,item:T) {
        list.push(item);
    }
    let mut shopping_list= Vec::new();
    add(& mut shopping_list,"milk");
}
