struct MyBox<T> {
    item: T,
}

// 1. 트레이트를 정의합니다.
trait Transformable<T> {
    // 미션:
    // - 메서드 이름: transform
    // - 제네릭 U, F를 사용하세요.
    // - self의 소유권을 가져갑니다.
    // - f는 FnOnce(T) -> U 인 클로저입니다.
    // - 결과로 MyBox<U>를 반환합니다.
    fn transform<U, F>(self, f: F) -> MyBox<U>
    where
        F: FnOnce(T) -> U;
}

// 2. MyBox에 대해 트레이트를 구현합니다.
impl<T> Transformable<T> for MyBox<T> {
    // 여기에 위에서 정의한 transform 메서드를 구현해 보세요!
    fn transform<U, F>(self, f: F) -> MyBox<U>
    where
        F: FnOnce(T) -> U,
    {
        /* 
            상황: 아이템들을 담고 있는 Box가 있고, 
            이 안에 든 아이템들을 하나씩 꺼내서 다른 타입으로 변환한 뒤, 
            다시 새로운 Box에 담는 트레이트를 설계해 봅시다.
            
            1. let box = Vec<T> 
            2. box.into_iter.map(|x| f(x)).collect::<Vec<U>>();
        */

        let convert_item = f(self.item);

        MyBox {
            item: convert_item
        }
    }
}
