fn main() {
    let  mut s = String::new();
    s = "19/2/4 3//4 3/4 43".into();

    let spaces : Vec<&str> = s.split(' ').collect();
    println!("{}",s);
    println!("'' {:?}",spaces);
    for val in spaces {
        let new : Vec<&str> = val.split('/').collect();
        println!("/ {:?}",new);
    }

}