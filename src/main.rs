fn sum_u32(numbers: &[u32]) -> Option<u32> {
    let mut sum = 0u32; // 定义一个变量来存储和，初始化为0
    for &number in numbers { // 遍历numbers切片中的每个数字
        match sum.checked_add(number) { // 尝试安全地添加数字
            Some(s) => sum = s, // 如果成功，则更新和
            None => return None, // 如果溢出，则立即返回None
        }
    }
    Some(sum) // 如果没有溢出，返回和的总数
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5]; // 测试数据
    match sum_u32(&numbers) {
        Some(sum) => println!("The sum is {}", sum),
        None => println!("Overflow occurred!"),
    }
}
