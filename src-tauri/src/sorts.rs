use cpu_time::ThreadTime;

pub fn bubble_sort(mut container: Vec<usize>) -> (Vec<usize>, f64) {
    let len = container.len();
    let slice = container.as_mut_slice();

    let start = ThreadTime::now();
    for i in 0..len - 1 {
        for j in 0..len - 1 - i {
            if slice[j] > slice[j + 1] {
                slice.swap(j, j + 1);
            }
        }
    }

    let full = start.elapsed();
    (container, full.as_secs_f64())
}

pub fn insert_sort(mut container: Vec<usize>) -> (Vec<usize>, f64) {
    let len = container.len();
    let slice = container.as_mut_slice();

    let start = ThreadTime::now();
    for i in 0..len {
        let mut min = i;
        for j in i + 1..len {
            if slice[j] < slice[min] {
                min = j;
            }
        }

        if min != i {
            slice.swap(i, min);
        }
    }

    let full = start.elapsed();
    (container, full.as_secs_f64())
}

pub fn cocktail_sort(mut container: Vec<usize>) -> (Vec<usize>, f64) {
    let mut begin = 0;
    let mut end = container.len() - 1;
    let slice = container.as_mut_slice();

    let start = ThreadTime::now();
    while begin < end {
        for i in begin..end {
            if slice[i] > slice[i + 1] {
                slice.swap(i, i + 1);
            }
        }

        end -= 1;
        for i in (begin + 1..=end).rev() {
            if slice[i] < slice[i - 1] {
                slice.swap(i, i - 1);
            }
        }

        begin += 1;
    }
    
    let full = start.elapsed();
    (container, full.as_secs_f64())
}

pub fn merge_sort(container: Vec<usize>) -> (Vec<usize>, f64) {
    let len = container.len();
    if len < 2 {
        (container, 0.0)
    } else {
        let (left, lt) = merge_sort(container[0..len / 2].to_vec());
        let (right, rt) = merge_sort(container[len / 2..].to_vec());
        let (merged, mt) = merge(left, right);
        (merged, lt + rt + mt)
    }
}

fn merge(left: Vec<usize>, right: Vec<usize>) -> (Vec<usize>, f64) {
    let mut i = 0;
    let mut j = 0;
    let mut merged = vec![];

    let start = ThreadTime::now();
    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            merged.push(left[i]);
            i += 1;
        } else {
            merged.push(right[j]);
            j += 1;
        }
    }

    if i < left.len() {
        merged.extend(left[i..].iter());
    }

    if j < right.len() {
        merged.extend(right[j..].iter());
    }

    let full = start.elapsed();
    (merged, full.as_secs_f64())
}

struct Bucket {
    hash: usize,
    values: Vec<usize>
}

impl Bucket {
    pub fn new(hash: usize, value: usize) -> Self {
        Self {
            hash,
            values: vec![value]
        }
    }
}

pub fn bucket_sort(container: Vec<usize>) -> (Vec<usize>, f64) {
    let mut buckets: Vec<Bucket> = vec![];

    let start = ThreadTime::now();
    for value in container.iter() {
        let hash = *value / 1000;

        match buckets.binary_search_by(|bucket| bucket.hash.cmp(&hash)) {
            Ok(index) => buckets[index].values.push(*value),
            Err(index) => buckets.insert(index, Bucket::new(hash, *value))
        }
    }

    let ret = buckets.into_iter().flat_map(|mut b| {
        b.values.sort();
        b.values
    }).collect();

    let full = start.elapsed();
    (ret, full.as_secs_f64())
}

pub fn radix_sort(mut container: Vec<usize>) -> (Vec<usize>, f64) {
    let mut radix = 0;
    let start = ThreadTime::now();
    while container.iter().any(|&v| v >> radix > 0) {
        container.sort_by_key(|&v| v >> radix & 15);
        radix += 4;
    }

    let full = start.elapsed();
    (container, full.as_secs_f64())
}

pub fn selection_sort(mut container: Vec<usize>) -> (Vec<usize>, f64) {
    let len = container.len();
    let slice = container.as_mut_slice();
    let start = ThreadTime::now();
    for i in 0..len {
        let mut min = i;
        for j in i + 1..len {
            if slice[j] < slice[min] {
                min = j;
            }
        }

        if i != min {
            slice.swap(i, min);
        }
    }

    let full = start.elapsed();
    (container, full.as_secs_f64())
}

pub fn comb_sort(mut container: Vec<usize>) -> (Vec<usize>, f64) {
    let shrink = 0.8;
    let len = container.len();
    let slice = container.as_mut_slice();
    let mut gap = len;
    let mut swapped = true;

    let start = ThreadTime::now();
    while gap > 1 || swapped {
        swapped = false;
        if gap > 1 {
            gap = (gap as f64 * shrink) as usize;
        }

        for i in 0..len - gap {
            if slice[i] > slice[i + gap] {
                slice.swap(i, i + gap);
                swapped = true;
            }
        }
    }

    let full = start.elapsed();
    (container, full.as_secs_f64())
}

const MARIN_ARRAY: [usize; 31] = [10376293531797946369, 4611686011984936961, 648518343925432321, 288230374541099009, 40532396042354689, 18014398106828801, 2533274639400961, 1125899806179329, 158329636651009, 70368719011841, 9895595212801, 4398040219649, 618472931329, 274876334081, 38654115841, 17179475969, 2415771649, 1073643521, 150958081, 67084289, 9427969, 4188161, 587521, 260609, 36289, 16001, 2161, 929, 109, 41, 1];
pub fn shell_sort(mut container: Vec<usize>) -> (Vec<usize>, f64) {
    let len = container.len();
    let slice = container.as_mut_slice();

    let start = ThreadTime::now();
    for gap in MARIN_ARRAY {
        let mut i = gap;
        while i < len {
            let mut j = i;
            while j >= gap && slice[j - gap] > slice[j] {
                slice.swap(j - gap, j);
                j -= gap;
            }

            i += 1;
        }
    }

    let full = start.elapsed();
    (container, full.as_secs_f64())
}

pub fn heap_sort(mut container: Vec<usize>) -> (Vec<usize>, f64) {
    let len = container.len();
    let slice = container.as_mut_slice();

    let start = ThreadTime::now();
    for i in (0..len / 2).rev() {
        max_heapify(slice, i, len - 1);
    }

    for i in (1..len).rev() {
        slice.swap(0, i);
        max_heapify(slice, 0, i - 1);
    }

    let full = start.elapsed();
    (container, full.as_secs_f64())
}

fn max_heapify(slice: &mut [usize], start: usize, end: usize) {
    let mut dad = start;
    let mut son = dad * 2 + 1;
    while son <= end {
        if son < end && slice[son] < slice[son + 1] {
            son += 1;
        }
        
        if slice[dad] > slice[son] {
            return;
        } else {
            slice.swap(dad, son);
            dad = son;
            son = dad * 2 + 1;
        }
    }
}

pub fn quick_sort(mut container: Vec<usize>) -> (Vec<usize>, f64) {
    let len = container.len();
    let slice = container.as_mut_slice();

    let start = ThreadTime::now();
    _quick_sort(slice, 0, (len - 1) as isize);

    let full = start.elapsed();
    (container, full.as_secs_f64())
}

fn _quick_sort(slice: &mut [usize], left: isize, right: isize) {
    if right > left {
        let new_index = partition(slice, left, right);
        _quick_sort(slice, left, new_index - 1);
        _quick_sort(slice, new_index + 1, right)
    }
}

fn partition(slice: &mut [usize], left: isize, right: isize) -> isize {
    let pindex = right;
    let value = slice[pindex as usize];
    let mut index = left;

    for i in left..right {
        if slice[i as usize] < value {
            slice.swap(index as usize, i as usize);
            index += 1;
        }
    }

    slice.swap(index as usize, right as usize);
    index
}