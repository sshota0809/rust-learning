async fn async_test() {
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    println!("async test!");
}

async fn async_test2() -> usize {
    tokio::time::sleep(tokio::time::Duration::from_millis(10000)).await;
    println!("async test2!");
    100
}

#[tokio::main]
async fn main() {
    // execute async_test()
    let at = tokio::spawn(async_test());
    println!("===pass===");
    //wait until async_test() complete
    at.await;
    println!("===pass===");
    //example to use join
    let mut handler = Vec::new();
    (1..=5).for_each(|_| {
        handler.push(tokio::spawn(async_test2()));
    });
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    println!("===pass===");
    let ret = futures::future::join_all(handler).await;
    println!("===pass===");
    let retval = ret.into_iter().map(|r| r.unwrap()).collect::<Vec<_>>();
    println!("{:?}", retval);
}
