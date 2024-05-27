fn main() {
    println!("Hello, world!");

    let nums = vec![2, 7, 11, 15];
    let target = 9;

    let val = two_sum(nums, target);

    println!("{:?}", val);

    let s = "Hello";
    let reversed = reverse(s);
    println!("{} reversed is {}", s, reversed);

    let foo = "palimpsest";
    println!("{} {}", foo, foo.chars().rev().collect::<String>());

    let cat = "cat";
    let reversed1 = reverse_alternate(cat);
    println!("{} spelt backwards is {}", cat, reversed1);
}

// index method not really clear tho
fn reverse_alternate(rev: &str) -> String {
    let chars: Vec<char> = rev.chars().collect();
    let mut rev_str = String::new();
    let mut rev_str_index = rev.len() - 1;
    loop {
        rev_str.push(chars[rev_str_index]);
        if rev_str_index == 0 {
            break;
        }
        rev_str_index -= 1;
    }
    rev_str
}

fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

//leet
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut current = 0;
    let mut indices = Vec::new();
    while current < nums.len() {
        for (i, val) in nums.iter().enumerate() {
            let key = nums[current];

            if i != current {
                if key + val == target {
                    indices.push(current as i32);
                    indices.push(i as i32);
                    break;
                }
            }
        }

        if !indices.is_empty() {
            break;
        }

        current += 1;
    }
    indices
}
