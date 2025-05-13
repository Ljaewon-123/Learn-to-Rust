fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

} // 스코프 가 끝나면 `drop`이 호출되어 메모리가 해제됨

// 힙 데이터까지 복사