use std::thread::scope;

#[tokio::main]
async fn main() {
    tokio::task_local! {
    static NUMBER: u32;
}
    // NUMBER.set(0, async move {
    //     assert_eq!(NUMBER.get(), 0);
    // }).await;
  let a  =   NUMBER.scope(1, async move {
        assert_eq!(NUMBER.get(), 1);
        println!("{:?}",NUMBER.get() );
        NUMBER.get()
    }).await;
    println!("{:?}", a);
    // let a = NUMBER.get();
    // println!("{:?}", a);
    NUMBER.scope(2, async move {
        assert_eq!(NUMBER.get(), 2);

        NUMBER.scope(3, async move {
            assert_eq!(NUMBER.get(), 3);
        }).await;
    }).await;
}