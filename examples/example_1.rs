use hass_rs::HassClient;
use uuid::Uuid;

static TOKEN: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiI0YzcyOGFjNDQ4MTc0NWIwODUxY2ZjMGE5YTc2ZWE1NSIsImlhdCI6MTU5NTIzNDYwMiwiZXhwIjoxOTEwNTk0NjAyfQ.Ow-mSTKNUSyqcJJrSBMYy6ftKMiTEwhMl-uhtBxln80";

#[cfg_attr(feature = "async-std-runtime", async_std::main)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = HassClient::new("localhost", 8123);
    client.connect(TOKEN).await?;
    let _num = Uuid::new_v4();
    let _payload = vec![1u8, 2, 3, 4];
   // let result = client.run(num, payload).await?;
   // println!("{:?}", result.0);

    Ok(())
}