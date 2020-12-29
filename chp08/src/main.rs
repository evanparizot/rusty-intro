fn main() {
    // 8.1

    let v: Vec<i32> = Vec::new();
    let mut y = vec![1,2,3];
    y.push(5);
    
    let x = vec![1, 2, 3, 4, 5];
    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);

    for i in &x {
        println!("{}", i);
    }


    // 8.2
    let mut s = String::new();
    


}
