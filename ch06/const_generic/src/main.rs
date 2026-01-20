use const_generic::Buffer;

fn main() {
    let buf = Buffer::from([0, 1, 2, 3, 4]);
    dbg!(buf);

    let group_of_seven = vec![
        "Canada",
        "France",
        "Germany",
        "Italy",
        "Japan",
        "United Kingdom",
        "United States",
        "European Union",
    ];
    let g7_buf: Buffer<_, 8> = Buffer::from(group_of_seven);
    dbg!(&g7_buf);
}
