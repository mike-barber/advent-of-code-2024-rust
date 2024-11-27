
fn main() -> anyhow::Result<()> {
    let text = common::read_file("input1.txt")?;
    println!("{}", text);
    Ok(())
}
