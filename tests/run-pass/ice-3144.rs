use std::collections::HashSet;

fn is_sync<T>(_: T) where T: Sync {}
fn is_send<T>(_: T) where T: Send {}

macro_rules! is_sync_send {
    ($ctor:expr, $iter:ident($($param:expr),+)) => ({
        let  x = $ctor;
        is_sync(x.$iter($( $param ),+));
        let  y = $ctor;
        is_send(y.$iter($( $param ),+));
    })
}

fn main() {
    is_sync_send!(HashSet::<usize>::new(), union(&HashSet::<usize>::new()));
}
