#[macro_export]
macro_rules! bimap {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {{
        use bimap::BiMap;
        #[allow(unused_mut)]
        let mut map = BiMap::new();
        $(map.insert($k, $v);)*
        map
    }};
}
