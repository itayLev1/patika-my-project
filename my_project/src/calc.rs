

// pub fn calculator(a: i32, sim: char, b: i32) {
//   let mut res: i32 = 0;



//   if sim == '+' {res = &a + &b}
//   else if sim == '-' {res =  &a - &b}
//   else if sim == '*' {res = &a * &b}
//   else if sim == '/' {res = &a / &b}

//   let res = res;
//   if res<0 {
//     return_minus(res);
//     return;
//   }

//   println!("Positive result is {}", &res);
  
//   fn return_minus (num: i32) {
//     println!("Negative result is {}", num);
//   }
// }


struct Calculation {
  a: i32,
  sym: char,
  b: i32,
  break_direction: bool,
  lightness: bool,
}

// struct EasyCalculation(i32, String, i32);


pub fn breakdown_calc(info: Calculation) {
  

  if info.break_direction == true {
    if info.lightness == true {info.sym = '+'}
    if info.lightness == false {info.sym = '*'}
  }
  if info.break_direction == false {
    if info.lightness == true {info.sym = '-'}
    if info.lightness == false {info.sym = '/'}
  }
  println!("{}{}{}", info.a, info.sym, info.b)
}


impl Calculation {
    fn add_symbol(char: char) -> char {

    }

    fn stringify(char: char) -> String {
      let string = String::from(char);
      string
    }
  }