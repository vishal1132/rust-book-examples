fn main() {
    let s1: String=String::from("hello");
    println!("{}",s1);
    let s2=s1; // s1 is moved to s2, so s1 is not valid anymore.
    
    let s3=s2.clone(); // s2 has been deep copied, so s2 is still valid after this. s1 can't be used to deep copy since s1 isn't even valid. 
    println!("{}",s3);
    println!("{}",s2);

    takes_ownership(s2); // after this s2 isn't valid anymore.
    
    let s4=give_ownership();
    println!("{}",s4);

    let s3=take_and_give_ownership(s3); // older s3 goes out of scope, this is the new s3 which is in scope.
    println!("{}",s3);

    let (len,s3)=calculate_length(s3); 
    println!("length of {} is {}",s3,len);

    let len=calculate_length_reference(&s3); // here ownership is not transferred, since the reference is passed but reference by default is also immutable.
    println!("Length of {} is {}",s3,len); 
   
    let mut s: String=String::from("new");
    
    change(&mut s);
    println!("{}",s)
}

/* 
    // this will not work, since the references are immutable by default as well.
    fn change(s: &String){
        s.push_str("abcd") 
    }
*/

fn change(s: &mut String){
    s.push_str("changed")
}

fn calculate_length_reference(s: &String)-> usize{
    s.len()
}

fn calculate_length(s: String)-> (usize,String){
    (s.len(),s)
    // (s,s.len()) wouldn't work since s is already moved therefore invalid and can't call len on invalid.
}

fn take_and_give_ownership(s: String)-> String{
    s
}

fn give_ownership() -> String {
    let s=String::from("giving ownership");
    s
}

fn takes_ownership(s: String){ // s comes into scope.
    println!("{}",s); // s is valid here.
} // s goes out of scope and `drop` is called. the backing memory is freed.

