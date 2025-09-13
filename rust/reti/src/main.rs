
fn main() {
    let ip = str_to_bin_octets("10.0.5.130");
    let subnet = str_to_bin_octets("255.255.255.192");


    println!("{:?}",ip );


    let f = bcast_bits(ip, subnet);
    let g = to_int_vec(f);
    println!("{:?}", g);


    let d = splitnet(23);
    println!("{:?}",d);
    let j = find_host(d);
    println!("{:?}", j);
}

fn find_host(n: u32) -> u32{
    2u32.pow(n) - 2
}


/*
Ritorna i bit dando il numero di sottoreti che si vuole usare
*/
fn splitnet(n: u32) -> u32 {
    assert!(n >= 1, "n deve essere >= 1");
    if n <= 1 { 0 } else { 32 - (n - 1).leading_zeros() }
}


/*

AND -> Per NETID
*/

fn netid(ip: Vec<Vec<u8>>, subnet: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    ip.iter()
        .zip(subnet.iter())
        .map(|(a, b)| {
            a.iter()
             .zip(b.iter())
             .map(|(x, y)| x & y) // AND bit a bit
             .collect()
        })
        .collect()
}

/*
OR NOT -> x il broadcast

*/

fn bcast_bits(ip_bits: Vec<Vec<u8>>, mask_bits: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    ip_bits
        .iter()
        .zip(mask_bits.iter())
        .map(|(oct_ip, oct_m)| {
            oct_ip
                .iter()
                .zip(oct_m.iter())
                .map(|(bit_ip, bit_m)| bit_ip | (1 - *bit_m)) // OR con NOT “booleano” della mask
                .collect()
        })
        .collect()
}




/*let classfull = classfull(binip);
    let typecf = findclassfull(classfull);
    println!("Tipo di classe:{:?}", typecf);
    */


/*Detect classfull*/

fn class_from_ip_first_octet(o1: u8) -> char {
    match o1 {
        1..=126   => 'A',         // 127 è loopback
        128..=191 => 'B',
        192..=223 => 'C',
        224..=239 => 'D',         // multicast
        _         => 'E',
    }
}




/*Conversione*/


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

