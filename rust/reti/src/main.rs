
fn main() {
    let ip = str_to_bin_octets("10.0.5.130");
    let subnet = str_to_bin_octets("255.255.255.192");


    println!("{:?}",ip );


    let f = andlogic(ip, subnet);
    let g = to_int_vec(f);
    println!("{:?}", g);


}


fn andlogic(netid: Vec<Vec<u8>>, subnet: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    netid.iter()
        .zip(subnet.iter())
        .map(|(a, b)| {
            a.iter()
             .zip(b.iter())
             .map(|(x, y)| x & y) // AND bit a bit
             .collect()
        })
        .collect()
}



/*let classfull = classfull(binip);
    let typecf = findclassfull(classfull);
    println!("Tipo di classe:{:?}", typecf);
    */
fn findclassfull(num: i32) -> char {
    if num == 0 {
        return 'C'
    }else if num == 1{
        return 'B'
    }else{
        return 'A'
    }
}


fn classfull(mut bits: Vec<Vec<u8>>) -> i32{
    for (idx, item) in bits.iter().rev().enumerate() {
        if item.contains(&1) {
            return idx as i32;
        }
    }
    -1
}

fn to_bin8(n: u8) -> Vec<u8> {
    (0..8).rev().map(|i| ((n >> i) & 1)).collect()
}

fn str_to_bin_octets(s: &str) -> Vec<Vec<u8>> {
    s.split('.')
        .map(|part| part.parse::<u8>().unwrap())
        .map(to_bin8)
        .collect()
}





fn to_int_vec(bits: Vec<Vec<u8>>) -> Vec<u32> {
    bits.into_iter()
        .map(|octet| {
            octet.into_iter().fold(0, |acc, bit| (acc << 1) | bit as u32)
        })
        .collect()
}

