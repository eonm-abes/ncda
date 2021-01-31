use ncda;

fn main() -> Result<(), ncda::NcdaError> {
    let unqualified_id = "cb32752361";
    println!(
        "The checksum char of {} is : {:?}",
        unqualified_id,
        ncda::checksum(unqualified_id)?
    );

    let qualified_id = "cb32752361d";
    println!(
        "{} is valid : {:?}",
        qualified_id,
        ncda::check(qualified_id).is_ok()
    );

    Ok(())
}
