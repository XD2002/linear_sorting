pub fn counting_sort(a: Vec<i8>) -> Vec<i8> {
    let k = a.iter().max().unwrap();
    let mut count = vec![0;*k as usize+1];

    let mut ret: Vec<i8> = vec![0;a.len()];

    for i in &a {
        count[*i as usize] += 1;
    }

    let mut til = 0;
    for i in 0..count.len() {
        count[i] += til;
        til = count[i];
    }

    let mut i: u8 = a.len() as u8;
    while i>0 {
        ret[count[a[i as usize - 1] as usize] as usize - 1] = a[i as usize - 1];
        count[a[i as usize - 1] as usize] -= 1;
        i -= 1;
    }

    ret
}
