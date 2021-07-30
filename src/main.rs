use stdext::function_name;

fn sync_fn() -> String {
    function_name!().to_owned()
}

async fn async_fn() -> String {
    function_name!().to_owned()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sync() {
        println!("(sync) function_name()! returns: {}", sync_fn());
        assert!(sync_fn().ends_with("sync_fn"));
    }

    #[tokio::test]
    async fn test_async() {
        println!("(async) function_name()! returns: {}", async_fn().await);
        assert!(async_fn().await.ends_with("async_fn"));
    }
}
