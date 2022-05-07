
fn text_finder(stri: &Vec<u32> ) -> u32 {
    let mut clone = stri.clone();
    clone.remove(0);
    if stri.len() > 1 {
        text_finder(&clone);
    }
    if stri[0] == 1 {println!("POUET"); 1  } else {0}   
}

fn main() {

    // let  mut s = String::new();
    // s = "19/2/4 3//4 3/4 43".into();

    // let spaces : Vec<&str> = s.split(' ').collect();
    // println!("{}",s);
    // println!("'' {:?}",spaces);
    // for val in spaces {
    //     let new : Vec<&str> = val.split('/').collect();
    //     println!("/ {:?}",new);
    // }

    let mut num = 0;
    
    num += text_finder(&vec![1,2,3,4,5,6,7,1,2,3,1,3,2,1]);
    println!("{}", num);

}