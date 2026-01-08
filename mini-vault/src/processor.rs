use std::fmt::Display;


struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    /// [ 미션: 아래 조건에 맞는 `map_value` 메서드를 설계하세요 ]
    /// 
    /// 1. 메서드 이름: `map_value`
    /// 2. 제네릭 파라미터: 
    ///    - `U`: 변환된 결과값의 타입입니다.
    ///    - `F`: 변환 로직을 담당하는 클로저 타입입니다.
    /// 3. 매개변수:
    ///    - `self`: 소유권을 가져와서 소비합니다. (기존 Container는 사라짐)
    ///    - `f`: `T` 타입을 받아 `U` 타입을 반환하는 클로저입니다.
    /// 4. 반환값: 
    ///    - 결과값 `U`를 새로 담은 `Container<U>`를 반환합니다.
    /// 5. 제약 조건 (where 절):
    ///    - `F`는 `FnOnce(T) -> U`를 만족해야 합니다. 
    ///      (소유권을 가져가서 한 번만 실행하면 되므로 FnOnce가 가장 적절합니다.)

    pub fn map_value<U,F>(self, f: F) -> Container<U> 
    where
        F: FnOnce(T) -> U
    {
        let new_value = f(self.value);
        Container { value: new_value }
    }

    pub fn display_process(&self) 
    where 
        T: Display 
    {
        println!("Value is: {}", self.value);
    }
}