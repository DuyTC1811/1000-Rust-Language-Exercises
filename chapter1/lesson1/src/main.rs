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
    
    let mut sum = 0;
    for n in 0..= number {
        sum += n;
    }

    print!("Tổng kết quả {}", sum);
}

