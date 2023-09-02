use observer::GameObserver;

mod observer;
pub mod probability;

fn main() {
    let mut observer = GameObserver::new(4, Some([0, 0, 0, 3, 0, 1, 2, 1, 0, 0, 0, 0, 0]));
    println!("-----\n{:?}", observer);
    observer.pickup();
    println!("-----\n{:?}", observer);
    observer.next();
    observer.pickup();
    println!("-----\n{:?}", observer);
    observer.pickup();
    println!("-----\n{:?}", observer);
    observer.query(2, 2, 1, false);
}
