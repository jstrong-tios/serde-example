# serde-example

## setup

```console
# install rust
$ curl https://sh.rustup.rs -sSf | sh
# clone repo
$ git clone https://github.com/jstrong-tios/serde-example.git
$ cd serde-example
# setup database
$ mysql -e 'create database serde_example'
$ echo 'DATABASE_URL=mysql://root@localhost/serde_example' > .env # or edit .env.example
# install diesel cli to run migration
$ cargo install diesel_cli
$ diesel migration run
# or just run the sql
$ mysql -p serde_example < migrations/2019-03-15-210155_create-example-table/up.sql
# run the program
$ cargo run
```

## Derive serialization/deserialization

Deriving `Serialize` and `Deserialize` gives you free serialization for structs, including nested, complicated ones.

```rust
#[derive(Serialize, Deserialize)]
pub struct CachedAlltimeRtValueStats {
  official_entity_id: i32,
  count_val: i32,
  // ... many other attributes
} 

// ...

fn main() -> Result<(), Box<std::error::Error>> {
    let mut a = CachedAlltimeRtValueStats::default();

    // json
    let json = serde_json::to_string_pretty(&a)?;
    println!("{}", json);
    let b = serde_json::from_str(&json)?;
    assert_eq!(a, b);
    
    // toml
    let toml = toml::to_string_pretty(&a)?;
    println!("{}", toml);
    let c = toml::from_str(&toml)?;
    assert_eq!(a, c);

    // yaml
    let yaml = serde_yaml::to_string(&a)?;
    println!("{}", yaml);
    let d = serde_yaml::from_str(&yaml)?;
    assert_eq!(a, d);

    // ...
```

## Diesel orm

Map structs to database tables easily.

`diesel` cli converts existing database schema to rust code definition:

```console
$ diesel print-schema > src/schema.rs
```

Also map to user-defined structs. Diesel has general orm capability but with type safety (optional compile time checks between struct and database table types, for example) and excellent performance.

```rust
use diesel::prelude::*;
use diesel::dsl::max;
use diesel::mysql::MysqlConnection;

mod schema; // where table definition lives
 
#[table_name = "mkt_cached_alltime_rt_value_stats"]
#[derive(Queryable, Insertable)]
pub struct CachedAlltimeRtValueStats {
  official_entity_id: i32,
  count_val: i32,
  // ... many other attributes
} 

// ...

fn main() -> Result<(), Box<std::error::Error>> {
    let conn = MysqlConnection::establish("mysql://root@localhost/test")?;

    // find last id to avoid inserting duplicate value on official_entity_id col
    let last_entity_id: Option<i32> =
        mkt_cached_alltime_rt_value_stats::table.select(
            max(mkt_cached_alltime_rt_value_stats::official_entity_id))
            .first(&conn)?;

    let mut a = CachedAlltimeRtValueStats::default();
    a.official_entity_id = last_entity_id.unwrap_or(0) + 1;

    diesel::insert_into(mkt_cached_alltime_rt_value_stats::table)
        .values(&a)
        .execute(&conn)?;

    // ... 
```
