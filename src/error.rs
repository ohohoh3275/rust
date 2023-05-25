
// 1. type 2. 

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn func_returning_result() -> Result<i32> {
    Ok(1)
}

fn main() {
    let _a = func_returning_result();

}