fn main() {
    let s1: String = String::from("hello");
    println!("{}", s1);
    let s2 = s1; // s1 is moved to s2, so s1 is not valid anymore.

    let s3 = s2.clone(); // s2 has been deep copied, so s2 is still valid after this. s1 can't be used to deep copy since s1 isn't even valid.
    println!("{}", s3);
    println!("{}", s2);

    takes_ownership(s2); // after this s2 isn't valid anymore.

    let s4 = give_ownership();
    println!("{}", s4);

    let s3 = take_and_give_ownership(s3); // older s3 goes out of scope, this is the new s3 which is in scope.
    println!("{}", s3);

    let (len, s3) = calculate_length(s3);
    println!("length of {} is {}", s3, len);

    let len = calculate_length_reference(&s3); // here ownership is not transferred, since the reference is passed but reference by default is also immutable.
    println!("Length of {} is {}", s3, len);

    let mut s: String = String::from("new");

    change(&mut s);
    let r1 = &mut s;
    println!("{}", r1);

    let r2 = &mut s;
    // println!("{}", r1); // can't do this since you can't have multiple mutable references
    println!("{}", r2);
    /*
        Can't use multiple mutable references simultaneously

        /*
            // This will cry for r2 reference.
        let r1=&mut s;
        println!("{}", r1);

        let r2=&mut s;
        println!("{}", r2);
        println!("{}", r1); // since r1 usage is not completely over before having r2 reference.
        */
    */
    // You can have multiple immutable references but not multiple mutable references simultaneously. Also you can't have mutable and immutable references simultaneously. Users of an immutable reference don’t expect the values to suddenly change out from under them!

    println!("{}", s);

    // Slices
    let mut s: String = String::from("some sentence that doesn't make sense.");
    let first_word_literal = first_word(&s);
    println!("first word is {}", first_word_literal);
    s.clear(); // is not permitted if line 56 is uncommented coz this borrows s as mutable, and therefore no immutable reference can be used later
    println!("after clear {}", s);
    // println!("first word is {}",first_word_literal);
    let s: String = String::from("some stupid string");
    let some = &s[0..4];
    let stupid = "stupid";
    let stupid: &str = &s[5..(5 + stupid.len())];
    println!("{}", some);
    println!("{}", stupid);

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    //`first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    let a = [1, 2, 3, 4, 5];
    let slice:&[i32]=&a[1..3];
    println!("{}",slice.len());
    for (i,&item) in slice.iter().enumerate(){
        println!("{},{}",i,item);
    }
    assert_eq!(slice,&[2,3]); // this panics if the assertion fails.

}

fn first_word(s: &str) -> &str {
    let barray = s.as_bytes();
    for (i, &item) in barray.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

/*
    // this returns the first occurrence of the space, which means the index where the first word ends.
    fn first_word(s: &String)->usize{
        let barray=s.as_bytes();
        for (i,&item) in barray.iter().enumerate(){
            if item==b' '{
                return i;
            }
        }
        s.len()
    }
*/
/*
// This will not be permitted, you can't create dangling references(references who are just there but pointing to nothing.)
fn dangle() -> &String {
    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
*/

/*
    // this will not work, since the references are immutable by default as well.
    fn change(s: &String){
        s.push_str("abcd")
    }
*/

fn change(s: &mut String) {
    s.push_str("changed")
}

fn calculate_length_reference(s: &String) -> usize {
    s.len()
}

fn calculate_length(s: String) -> (usize, String) {
    (s.len(), s)
    // (s,s.len()) wouldn't work since s is already moved therefore invalid and can't call len on invalid.
}

fn take_and_give_ownership(s: String) -> String {
    s
}

fn give_ownership() -> String {
    let s = String::from("giving ownership");
    s
}

fn takes_ownership(s: String) {
    // s comes into scope.
    println!("{}", s); // s is valid here.
} // s goes out of scope and `drop` is called. the backing memory is freed.
