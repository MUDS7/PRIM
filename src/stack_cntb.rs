use std::ptr::null;

#[derive(Debug)]
pub struct Stack<T> {
    pub data: Vec<T>,
    pub top: usize,
}

impl <T> Stack<T> {
    pub fn new(size: usize) -> Self {
        Stack {
            data: Vec::with_capacity(size),
            top: 0,
        }
    }

    pub fn push(&mut self, item: T) -> Result<(), String> {
        if self.top >= self.data.capacity() {
            return Err(String::from("There is no space in stack!"))
        }

        self.data.push(item);
        self.top += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            return None
        }

        self.top -= 1;
        self.data.pop()
    }

    pub fn top(&self) -> usize {
        self.top
    }

}
#[test]
fn test_stack(){
    let mut s=Stack::new(4);
    s.push("(");
    //s.pop();
    let val=s.top;
    if val==0 {
        println!("val={:?}", val);
    }
}