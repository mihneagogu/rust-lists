mod lib;
fn main() {
    use lib::lists::Stack;
    let mut stack :Stack<u32> = Stack::new(); 


    stack.push(1);
    stack.push(2);
    stack.push(3);


    stack.push(4);
    stack.push(5);


}
