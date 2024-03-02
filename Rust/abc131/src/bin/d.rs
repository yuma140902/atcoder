fn main() {
    proconio::input! {
        n: usize,
        input: [u32; 2*n]
    }
    let mut tasks = Vec::with_capacity(n);
    for i in (0..2 * n).step_by(2) {
        tasks.push(Task {
            time: input[i],
            deadline: input[i + 1],
        });
    }
    tasks.sort_by_key(|task| task.rest());
    dbg!(&tasks);

    for i in 0..tasks.len() {
        if tasks[i].rest() < 0 {
            println!("No");
            return;
        }
        for k in i..tasks.len() {
            tasks[k].time += tasks[i].time;
        }
    }

    println!("Yes");
}

#[derive(Debug)]
struct Task {
    time: u32,
    deadline: u32,
}

impl Task {
    fn rest(&self) -> i32 {
        self.deadline as i32 - self.time as i32
    }
}
