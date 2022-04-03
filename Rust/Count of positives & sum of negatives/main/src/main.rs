fn main() {
    const X: usize = 15;
    let mut i: usize = 0;
    let array:[i16; X]=[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14, -15];
    let mut val: [i16; 2]=[0,0];

    while i < X {
        if array[i]>0 {
            val[0] += 1;
        }
        else if array[i]<0 {
            val[1] += array[i];
        }
        i+=1;
    }
    
    println!("{}", val[0]);
    println!("{}", val[1]);
}
