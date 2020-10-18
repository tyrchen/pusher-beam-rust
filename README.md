# Pusher

Pusher BEAMS support. See: https://pusher.com/docs/beams.

Example:

```rust
    let instance_id = env::var("PUSHER_BEAM_INSTANCE_ID").unwrap();
    let secret = env::var("PUSHER_BEAM_SECRET").unwrap();
    let pusher = PusherBeam::new(&instance_id, &secret);
    let request = r#"
    {"web":{"notification":{"title":"Hello","body":"Hello, world!"}}}
    "#;
    let payload = Payload {
        interests: vec!["hello".to_owned(), "hi".to_owned()],
        web: serde_json::from_str(request)?,
        fcm: Value::Null,
        apns: Value::Null,
    };
    pusher.publish(&payload).await.unwrap();
```
