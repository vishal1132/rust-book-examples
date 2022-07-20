
fn main() {
    println!("Hello, world!");
    
    /*
    # HashMaps
    Just like vectors, hash maps store their data on the heap
    HashMaps have to be homogenous, all keys of same type and all values of same type.
    */
    use std::collections::HashMap;
 
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    // zipping two vectors together, forming a tuple and then collecting that tuple into hashmap.
    // HashMap<_,_> type annotation is needed, since it's possible to collect into various data structures. _,_ coz HashMap now can infer automatically the type of key and value.
    let mut scores:HashMap<_,_> =teams.into_iter().zip(initial_scores.into_iter()).collect();
    // For types that implement the Copy trait, like i32, the values are copied into the hash map. For owned values like String, the values will be moved and the hash map will be the owner of those values, example-
    let a=String::from("abcd");
    let b=String::from("efgh");
    let mut h=HashMap::new();
    h.insert(a, b);
    
    // println!("{}",a); uncommenting this would result in compilation errors, since a is moved, ownership is transferred, it can't be used anymore.

    // to iterate over HashMap
    for (k,v) in &scores{
        println!("{}, {}", k, v);
    }
    // this will return an Option, so we will have to match on Some and None.
    // get takes by an immutable reference so the value is not moved and can be used after the below line as well.
    let team_name=String::from("Blue");
    let score=scores.get(&team_name);
    match score{
        Some(a)=>println!("{}",a),
        None=> println!("invalid"),
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    // only add if the key doesn't exist in the map, or it doesn't have a value in the map.
    // or_insert returns a mutable reference to the value present.
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
    // to check how many times a word occurs in a string.

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // count is a mutable reference to the value in hashmap, and we are incrementing it by 1 each time we encounter a word.
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
    // By default, HashMap uses a hashing function called SipHash that can provide resistance to Denial of Service (DoS) attacks involving hash tables
    //TODO: do the exercises mentioned in the text.
}
