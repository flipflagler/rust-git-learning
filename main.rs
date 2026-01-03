trait Summary {
    fn summary(&self)-> String {
        return String :: from ("hi there");
    }
}
struct User {
    name :String,
    age :u32,
}
impl Summary for User{
    fn summary(&self)-> String {
        print!("hi");
        return format!("thename is {},and the age is {}",self.name,self.age);
    }
}
fn main(){
    let user = User{
        name : String::from("harsh"),
        age:21,
    };
    println!("{}",user.summary());
}