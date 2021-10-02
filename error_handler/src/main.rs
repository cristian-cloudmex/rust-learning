fn main() {

    // Option enum (None, Some(T))
    let numbers = vec![1,2,3,4];

    let number_1 = numbers.get(0..2);
    let number_2 = numbers.get(99);

    println!("{:?}", number_1);
    println!("{:?}", number_2);

    let m = String::from("Hola");
    let x = m.clone();
    println!("{} {}", x, m);
    println!("{:p} {:p}", &x, &m);
}
