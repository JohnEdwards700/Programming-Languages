/* fn main() {
    println!("Hello, world!");
}

fn use_case_one(){
    let mut value = 42;
    let mut not_mutating  = value;
    increment_by_three(&mut value);
    not_mutating +=2;
    println!("{value} {not_mutating}")
}

fn use_case_two(){
    let mut value = 42;
    let mutable = &mut value;
    increment_by_three(mutable)
    *mutable += 2;
    println!("{value} {mutable}")
}

fn increment_by_three(value: &mut 132){
    *value += 3
}

fn main(){
    int mut vector = Vec::new();
    vector.push((String::from("foo"), 42));
    insert(String::from("hey"), 43, &mut vector)
}

fn insert(s: String, v: i32, vec: &mut Vec<(String, i32)>){
    vec.push((s, v));
}

*/

fn middle_slice_array(source: &[i32]) -> &[i32] {
    let len = source.len();
    &source[len / 2..=len / 2 * 1]
}

fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let r = middle_slice_array(&arr);
    println!("{}, {}"r[0], r[1])
}
