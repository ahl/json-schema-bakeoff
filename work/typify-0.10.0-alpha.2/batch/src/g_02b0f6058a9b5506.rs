
pub fn __conformance_check(v: ::serde_json::Value) -> ::std::result::Result<(), ::std::string::String> {
    ::serde_json::from_value::<:: serde_json :: Map < :: std :: string :: String , :: serde_json :: Value >>(v).map(|_| ()).map_err(|e| e.to_string())
}
