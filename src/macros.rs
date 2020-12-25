#[macro_export]
macro_rules! headers{
    { $($key:expr => $value:expr),+ } => {
        {
            use web_sys::Headers;
            let headers = Headers::new().unwrap();
            $(
                headers.set($key, $value).unwrap();
            )+
            headers
        }
     };
     () => { Headers::new().unwrap() };
}
