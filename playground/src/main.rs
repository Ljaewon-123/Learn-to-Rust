fn main() {
    let mut s = String::from("hello"); // 필요한 만큼만 메모리 요구 

    s.push_str(", world!"); // push_str()이 문자열에 리터럴을 추가합니다

    println!("{}", s); // 이 줄이 `hello, world!`를 출력합니다

} // 스코프 가 끝나면 `drop`이 호출되어 메모리가 해제됨

