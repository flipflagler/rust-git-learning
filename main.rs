fn main (){
    let  v1 = vec![1,2,3,4];
    let iter = v1.iter();
    let iter2 =  iter.filter(|x| *x%2 != 0).map(|x| x*2);
    let v2:Vec<i32> = iter2.collect();
    // for x in iter2 {
    //     v2.push(x);
    // }
   println!("{:?}",v2);
}