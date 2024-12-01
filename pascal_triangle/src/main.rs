fn main() {
    let ans = pascal_triangle(30);

    println!("{:?}", ans);
}

fn pascal_triangle(n: i32) -> Vec<Vec<i32>> {
    let mut triangle = vec![vec![1]];

    for i in 1..n as usize {
        let mut row = vec![1];
        for j in 1..i {
            row.push(triangle[i - 1][j - 1] + triangle[i - 1][j]);
        }
        row.push(1);
        triangle.push(row);
    }

    return triangle;
}
