fn main() {
    let rect1 = (30, 50);

    println!(
        "area of rectangle: {} square pixel",
        area(rect1)
    );
}

fn area(demensions: (u32, u32)) -> u32 {
    // 인덱스 0은 너비, 1은 높이를 외워야 한다. 직관적이지 않다
    demensions.0 * demensions.1
}