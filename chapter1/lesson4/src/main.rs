use std::io::stdin;

fn main() {
    let mut input = String::new();

    println!("Nhập vào một số nguyên: ");
    stdin().read_line(&mut input).expect("Không thể đọc dữ liệu");

    let number: i32 = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            print!("Không thể chuyển đổi chuỗi {} thành số nguyên. \n", input.trim());
            return;
        }
    };

    if number <= 1 {
        println!("Số phải lớn hơn 1");
        return;
    }

    let mut sum = 0.0;
    for i in 1..= number {
        sum += 1.0 / (2 * i) as f64;
    }

    println!("Tổng = {:.2}", sum);
}

