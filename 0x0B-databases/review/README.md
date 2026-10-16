# Review: "Holdings repository"

A teammate opens this PR: *"Adds the repository the server uses for qualification records. Works with the seed data."* Review it.

```rust
pub async fn holdings_for_station(pool: &PgPool, station: &str) -> Vec<(i32, String, String)> {
    let staff: Vec<(i32, String)> = sqlx::query_as(&format!(
        "SELECT staff_number, name FROM employees WHERE station_code = '{station}'"
    ))
    .fetch_all(pool)
    .await
    .unwrap();

    let mut result = Vec::new();
    for (number, name) in staff {
        let codes: Vec<(String,)> = sqlx::query_as("SELECT code FROM holdings WHERE staff_number = $1")
            .bind(number)
            .fetch_all(pool)
            .await
            .unwrap();
        for (code,) in codes {
            result.push((number, name.clone(), code));
        }
    }
    result
}

pub async fn renew(pool: &PgPool, staff_number: i32, code: &str, completed_on: NaiveDate) {
    sqlx::query("DELETE FROM holdings WHERE staff_number = $1 AND code = $2")
        .bind(staff_number).bind(code).execute(pool).await.ok();
    sqlx::query("INSERT INTO holdings (staff_number, code, completed_on, valid_until) VALUES ($1, $2, $3, $3 + interval '24 months')")
        .bind(staff_number).bind(code).bind(completed_on).execute(pool).await.ok();
}
```
