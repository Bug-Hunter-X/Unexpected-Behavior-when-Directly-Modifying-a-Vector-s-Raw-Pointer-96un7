fn main() {
    let mut v = vec![1, 2, 3];
    unsafe {
        *v.as_mut_ptr() = 4; //This is still unsafe but correct in this context
    }
    println!("v: {:?}", v); // Output: v: [4, 2, 3]

    //Better, safer approach
    v[0] = 4;
    println!("v: {:?}", v); // Output: v: [4, 2, 3]
} 