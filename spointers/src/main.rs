
trait Number {
    fn print(&self)->();
}

struct One{
    x:u32,
    name:String
}
struct Two{
    x:u32,
    name:Vec<String>
}

impl Number for One {
    fn print(&self)->() {
        println!("1");
    }
}

impl Number for Two {
    fn print(&self)->() {
        println!("2");
    }
}


fn main() {    
    let mut one: Box<dyn Number>;
    one = Box::new(One{x:3,name:"somename".to_string()});
    one.print();
    one = Box::new(Two{x:2,name:vec!["somename".to_string()]});
    one.print();
}
