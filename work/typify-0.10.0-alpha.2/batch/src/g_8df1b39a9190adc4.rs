
pub fn __conformance_check(v: ::serde_json::Value) -> ::std::result::Result<(), ::std::string::String> {
    ::serde_json::from_value::<i64>(v).map(|_| ()).map_err(|e| e.to_string())
}
