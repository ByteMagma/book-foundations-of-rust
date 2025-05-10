fn main() {
    let x = 100;
    if x >= 90 {
        println!("Great score, you got a {}", x);
    } else if x >= 80 && x < 90  {
        println!("Pretty good, you got a {}", x);
    }  else if x >= 70 && x < 80  {
        println!("Hmm... looks like you need to study more, you got a {}", x);
    }  else if x >= 60 && x < 70  {
        println!("Ouch, you should get a tutor, you got a {}", x);
    }  else {
        println!("Yo, stop bingeing on Netflix, you got a {}", x);
    }
}

