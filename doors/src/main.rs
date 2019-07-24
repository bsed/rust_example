fn main() {
    println!("Hello, world!");
    let mut doors = [false; 100];
    solve(&mut doors);

    for(idx, door) in doors.iter().enumerate() {
        println!("door {} open: {}", idx + 1, door);
    }
}

fn solve(doors: &mut [bool]) {
    for pass in 1..101 {
        let mut p = pass;
        while p <= 100 {
            doors[p-1] = !doors[p -1];
            p += pass;
        }
    }
}

#[test]
fn solution() {
    let mut doors = [false; 100];
    solve(&mut doors);

    for i in 1..11 {
        assert!(doors[i * i - 1]);
    }
}