const VALID_ADDRESSES: [&'static str; 2] = ["/", "/index.html"];

pub fn validate(address: &str) -> Result<(), &'static str> {
    if VALID_ADDRESSES.contains(&address) {
        return Ok(());
    }

    return Err("Invalid GET address");
}
