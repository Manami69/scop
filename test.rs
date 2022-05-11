
// fn text_finder(stri: &Vec<u32> ) -> u32 {
//     let mut clone = stri.clone();
//     clone.remove(0);
//     if stri.len() > 1 {
//         text_finder(&clone);
//     }
//     if stri[0] == 1 {println!("POUET"); 1  } else {0}   
// }

fn main() {
   let p = "32.3//232".to_string();
   let v  = p.iter().take_while(|c| !c.is_digit()).collect();
	println!("|{}|", p.parse::<usize>().unwrap());
}