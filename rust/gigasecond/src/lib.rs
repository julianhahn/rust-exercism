use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    // reminder for myself about test assertions: left = "expected value", right = "actual value"

    // i just don't really know what 'saturating' means? It clamps at max if it reaches the max of an primitive dateTime? thats how it looked from the assertions in the type description
    // start.saturated_add ...
    let new_date = start.checked_add(time::Duration::seconds(1000_000_000));
    match new_date {
        Some(input) => {
            println!("New time: {}", input);
            input
        }
        None => {
            println!("overflow! return same as start: {}", start);
            start
        }
    }
}
