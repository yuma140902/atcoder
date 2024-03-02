fn main() {
    proconio::input! {
        n: u32,
        q: u32,
        xs: [u32; q]
    }

    let mut vec = vec![];
    for i in 0..n {
        vec.push(i + 1);
    }

    let mut is = vec![];
    for i in 0..n as usize {
        set_index(&mut is, i as u32 + 1, i);
    }

    for x in xs {
        let mut i = get_index(&is, x);
        i = if i + 1 == vec.len() { i - 1 } else { i };

        let i0 = get_index(&is, vec[i]);
        let i1 = get_index(&is, vec[i + 1]);
        set_index(&mut is, vec[i], i1);
        set_index(&mut is, vec[i + 1], i0);

        let tmp = vec[i];
        vec[i] = vec[i + 1];
        vec[i + 1] = tmp;
    }

    for v in vec {
        print!("{} ", v);
    }
}

fn get_index(is: &Vec<usize>, x: u32) -> usize {
    is[x as usize - 1]
}

fn set_index(is: &mut Vec<usize>, x: u32, i: usize) {
    is[x as usize - 1] = i;
}
