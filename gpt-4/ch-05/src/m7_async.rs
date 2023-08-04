async fn mt_asunc_call(url: &str) -> Result<serde_json::Value, reqwest::Error> {
    //
    // let respose: serde_json::Value = reqwest::get(url).await;
    let respose: serde_json::Value = reqwest::get(url).await?.json::<serde_json::Value>().await?;

    Ok(respose)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_async() {
        //dbg!("Hello tests_async!!!");

        let api_url = "https://leon.ru/api-2/betline/event/all?ctag=ru-RU&eventId=1970324842247793";
        let my_res = mt_asunc_call(api_url).await;

        match my_res {
            Ok(r) => dbg!(r),
            Err(_) => {
                panic!("Error reqwest");
            }
        };
    }
}
