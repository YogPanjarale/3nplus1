// extern crate big_unsigned_ints};

fn main() {
    // let n = big_unsigned_ints::U2048::new();
    let mut n :i128= 2;
    n= n.pow(68);
    println!("{}", n);
    let mut i:i128 = 0;
    while i < n{
        if i%1_00_000_000==0{
            print!("{} ",i);
        }
        i+=1;
    }
}
//295147905179352825856
//16:47:10 0
//16:54:30 13105000000
