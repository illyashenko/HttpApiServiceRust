use waiter_di::*;

pub fn id_default() -> i32 {
    0
}

pub fn get<T>()->Container<T>{
   let cont = Container::<T>::new();
    cont
}