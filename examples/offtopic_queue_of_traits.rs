trait Greeter: std::fmt::Debug {
    fn greet(&mut self) -> String;
}

#[derive(Default, Debug)]
struct Dog {
    barks_count: usize,
}

#[derive(Default, Debug)]
struct Human {
    hello_count: usize,
}

impl Greeter for Dog {
    fn greet(&mut self) -> String {
        self.barks_count += 1;
        format!("woof #{:#?}", self.barks_count)
    }
}

impl Greeter for Human {
    fn greet(&mut self) -> String {
        self.hello_count += 1;
        format!("hello #{:#?}", self.hello_count)
    }
}

#[derive(Default, Debug)]
struct Greeters {
    greeters: Vec<Box<dyn Greeter>>
}

impl Greeters {
    fn push(&mut self, greeter: Box<dyn Greeter>) {
        self.greeters.push(greeter);
    }

    fn peek_mut(&mut self) -> &mut Box<dyn Greeter> {
        self.greeters.last_mut().expect("oops")
    }

    fn pop(&mut self) -> Box<dyn Greeter> {
        self.greeters.pop().expect("oops")
    }
}

fn main() {
    let mut greeters = Greeters::default();    
    
    let to_push = Box::from(Dog::default());
    println!("pushing: {:#?}", to_push);
    greeters.push(to_push);
    println!("{}", greeters.peek_mut().greet());

    let to_push = Box::from(Human::default());
    println!("pushing: {:#?}", to_push);
    greeters.push(to_push);
    println!("{}", greeters.peek_mut().greet());

    println!("poped: {:#?}", greeters.pop());
    println!("{}", greeters.peek_mut().greet());
    println!("poped: {:#?}", greeters.pop());
}