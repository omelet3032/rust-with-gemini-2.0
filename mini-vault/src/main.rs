struct Transformer<T> {
    data: Vec<T>,
}

impl<T> Transformer<T> {
    /// [ 연습 미션 ]
    /// 1. 메서드 이름: `process_and_report`
    /// 2. 제네릭 타입 파라미터: 
    ///    - `U`: 변환 결과의 타입
    ///    - `F`: 변환 로직을 담은 클로저 타입
    /// 3. 매개변수:
    ///    - `self`: 소유권을 가져와서 소비합니다. (into_iter 느낌)
    ///    - `f`: `T`를 받아서 `U`를 반환하는 함수(Fn)입니다.
    /// 4. 반환값: 
    ///    - 변환된 아이템들(`U`)을 담은 새로운 `Vec<U>`를 반환합니다.
    /// 5. 제약 조건(where):
    ///    - `F`는 `Fn(T) -> U` 트레이트를 만족해야 합니다.
    
    // 여기에 코드를 작성해 보세요!
    pub fn process_and_report<U,F> (self, f: F) -> Vec<U> 
    where 
        F: Fn(T)  -> U
    {
        self.data.into_iter().map(f).collect()
    }
}
fn main() {
    println!("Hello, world!");
}
