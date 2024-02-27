fn main() {
    let s1 = String::from("helo");
    takes_ownership(s1);
    // println!("{}",s1); //this will cause an error since s1 isn't valid anymore. it was dropped
}

fn takes_ownership(some_string: String){
    println!("I've borrowed {} from s1 in my main function and its scope ends here", some_string);
}