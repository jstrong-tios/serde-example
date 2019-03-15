#![recursion_limit="256"]
#![allow(unused_imports)]
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;

use chrono::{DateTime, Utc, NaiveDateTime};
use schema::mkt_cached_alltime_rt_value_stats;

mod schema;

#[table_name = "mkt_cached_alltime_rt_value_stats"]
#[derive(Clone, PartialEq, Debug, Queryable, Insertable, Serialize, Deserialize)]
pub struct CachedAlltimeRtValueStats {
  official_entity_id: i32,
  count_val: i32,
  min_val: f32,
  max_val: f32,
  avg_val: f32,
  avg_abs_val: f32,
  variance_val: f32,
  stddev_val: f32,
  est_pct_01_val: f32,
  est_pct_05_val: f32,
  est_pct_10_val: f32,
  est_pct_25_val: f32,
  est_pct_75_val: f32,
  est_pct_90_val: f32,
  est_pct_95_val: f32,
  est_pct_99_val: f32,
  count_val_30d: i32,
  min_val_30d: f32,
  max_val_30d: f32,
  avg_val_30d: f32,
  avg_abs_val_30d: f32,
  variance_val_30d: f32,
  stddev_val_30d: f32,
  est_pct_01_val_30d: f32,
  est_pct_05_val_30d: f32,
  est_pct_25_val_30d: f32,
  est_pct_75_val_30d: f32,
  est_pct_95_val_30d: f32,
  est_pct_99_val_30d: f32,
  count_val_90d: i32,
  min_val_90d: f32,
  max_val_90d: f32,
  avg_val_90d: f32,
  avg_abs_val_90d: f32,
  variance_val_90d: f32,
  stddev_val_90d: f32,
  est_pct_01_val_90d: f32,
  est_pct_05_val_90d: f32,
  est_pct_25_val_90d: f32,
  est_pct_75_val_90d: f32,
  est_pct_95_val_90d: f32,
  est_pct_99_val_90d: f32,
  count_val_365d: i32,
  min_val_365d: f32,
  max_val_365d: f32,
  avg_val_365d: f32,
  avg_abs_val_365d: f32,
  variance_val_365d: f32,
  stddev_val_365d: f32,
  est_pct_01_val_365d: f32,
  est_pct_05_val_365d: f32,
  est_pct_25_val_365d: f32,
  est_pct_75_val_365d: f32,
  est_pct_95_val_365d: f32,
  est_pct_99_val_365d: f32,
  onevent_count_val: i32,
  onevent_min_val: f32,
  onevent_max_val: f32,
  onevent_avg_val: f32,
  onevent_avg_abs_val: f32,
  onevent_variance_val: f32,
  onevent_stddev_val: f32,
  onevent_est_pct_01_val: f32,
  onevent_est_pct_05_val: f32,
  onevent_est_pct_10_val: f32,
  onevent_est_pct_25_val: f32,
  onevent_est_pct_75_val: f32,
  onevent_est_pct_90_val: f32,
  onevent_est_pct_95_val: f32,
  onevent_est_pct_99_val: f32,
  onevent_count_val_30d: i32,
  onevent_min_val_30d: f32,
  onevent_max_val_30d: f32,
  onevent_avg_val_30d: f32,
  onevent_avg_abs_val_30d: f32,
  onevent_variance_val_30d: f32,
  onevent_stddev_val_30d: f32,
  onevent_est_pct_01_val_30d: f32,
  onevent_est_pct_05_val_30d: f32,
  onevent_est_pct_25_val_30d: f32,
  onevent_est_pct_75_val_30d: f32,
  onevent_est_pct_95_val_30d: f32,
  onevent_est_pct_99_val_30d: f32,
  onevent_count_val_90d: i32,
  onevent_min_val_90d: f32,
  onevent_max_val_90d: f32,
  onevent_avg_val_90d: f32,
  onevent_avg_abs_val_90d: f32,
  onevent_variance_val_90d: f32,
  onevent_stddev_val_90d: f32,
  onevent_est_pct_01_val_90d: f32,
  onevent_est_pct_05_val_90d: f32,
  onevent_est_pct_25_val_90d: f32,
  onevent_est_pct_75_val_90d: f32,
  onevent_est_pct_95_val_90d: f32,
  onevent_est_pct_99_val_90d: f32,
  onevent_count_val_365d: i32,
  onevent_min_val_365d: f32,
  onevent_max_val_365d: f32,
  onevent_avg_val_365d: f32,
  onevent_avg_abs_val_365d: f32,
  onevent_variance_val_365d: f32,
  onevent_stddev_val_365d: f32,
  onevent_est_pct_01_val_365d: f32,
  onevent_est_pct_05_val_365d: f32,
  onevent_est_pct_25_val_365d: f32,
  onevent_est_pct_75_val_365d: f32,
  onevent_est_pct_95_val_365d: f32,
  onevent_est_pct_99_val_365d: f32,
  //updated_at: DateTime<Utc>,
  updated_at: NaiveDateTime,
}

impl Default for CachedAlltimeRtValueStats {
    fn default() -> Self {
        Self {
            official_entity_id: 0,
            count_val: 0,
            min_val: 0.0,
            max_val: 0.0,
            avg_val: 0.0,
            avg_abs_val: 0.0,
            variance_val: 0.0,
            stddev_val: 0.0,
            est_pct_01_val: 0.0,
            est_pct_05_val: 0.0,
            est_pct_10_val: 0.0,
            est_pct_25_val: 0.0,
            est_pct_75_val: 0.0,
            est_pct_90_val: 0.0,
            est_pct_95_val: 0.0,
            est_pct_99_val: 0.0,
            count_val_30d: 0,
            min_val_30d: 0.0,
            max_val_30d: 0.0,
            avg_val_30d: 0.0,
            avg_abs_val_30d: 0.0,
            variance_val_30d: 0.0,
            stddev_val_30d: 0.0,
            est_pct_01_val_30d: 0.0,
            est_pct_05_val_30d: 0.0,
            est_pct_25_val_30d: 0.0,
            est_pct_75_val_30d: 0.0,
            est_pct_95_val_30d: 0.0,
            est_pct_99_val_30d: 0.0,
            count_val_90d: 0,
            min_val_90d: 0.0,
            max_val_90d: 0.0,
            avg_val_90d: 0.0,
            avg_abs_val_90d: 0.0,
            variance_val_90d: 0.0,
            stddev_val_90d: 0.0,
            est_pct_01_val_90d: 0.0,
            est_pct_05_val_90d: 0.0,
            est_pct_25_val_90d: 0.0,
            est_pct_75_val_90d: 0.0,
            est_pct_95_val_90d: 0.0,
            est_pct_99_val_90d: 0.0,
            count_val_365d: 0,
            min_val_365d: 0.0,
            max_val_365d: 0.0,
            avg_val_365d: 0.0,
            avg_abs_val_365d: 0.0,
            variance_val_365d: 0.0,
            stddev_val_365d: 0.0,
            est_pct_01_val_365d: 0.0,
            est_pct_05_val_365d: 0.0,
            est_pct_25_val_365d: 0.0,
            est_pct_75_val_365d: 0.0,
            est_pct_95_val_365d: 0.0,
            est_pct_99_val_365d: 0.0,
            onevent_count_val: 0,
            onevent_min_val: 0.0,
            onevent_max_val: 0.0,
            onevent_avg_val: 0.0,
            onevent_avg_abs_val: 0.0,
            onevent_variance_val: 0.0,
            onevent_stddev_val: 0.0,
            onevent_est_pct_01_val: 0.0,
            onevent_est_pct_05_val: 0.0,
            onevent_est_pct_10_val: 0.0,
            onevent_est_pct_25_val: 0.0,
            onevent_est_pct_75_val: 0.0,
            onevent_est_pct_90_val: 0.0,
            onevent_est_pct_95_val: 0.0,
            onevent_est_pct_99_val: 0.0,
            onevent_count_val_30d: 0,
            onevent_min_val_30d: 0.0,
            onevent_max_val_30d: 0.0,
            onevent_avg_val_30d: 0.0,
            onevent_avg_abs_val_30d: 0.0,
            onevent_variance_val_30d: 0.0,
            onevent_stddev_val_30d: 0.0,
            onevent_est_pct_01_val_30d: 0.0,
            onevent_est_pct_05_val_30d: 0.0,
            onevent_est_pct_25_val_30d: 0.0,
            onevent_est_pct_75_val_30d: 0.0,
            onevent_est_pct_95_val_30d: 0.0,
            onevent_est_pct_99_val_30d: 0.0,
            onevent_count_val_90d: 0,
            onevent_min_val_90d: 0.0,
            onevent_max_val_90d: 0.0,
            onevent_avg_val_90d: 0.0,
            onevent_avg_abs_val_90d: 0.0,
            onevent_variance_val_90d: 0.0,
            onevent_stddev_val_90d: 0.0,
            onevent_est_pct_01_val_90d: 0.0,
            onevent_est_pct_05_val_90d: 0.0,
            onevent_est_pct_25_val_90d: 0.0,
            onevent_est_pct_75_val_90d: 0.0,
            onevent_est_pct_95_val_90d: 0.0,
            onevent_est_pct_99_val_90d: 0.0,
            onevent_count_val_365d: 0,
            onevent_min_val_365d: 0.0,
            onevent_max_val_365d: 0.0,
            onevent_avg_val_365d: 0.0,
            onevent_avg_abs_val_365d: 0.0,
            onevent_variance_val_365d: 0.0,
            onevent_stddev_val_365d: 0.0,
            onevent_est_pct_01_val_365d: 0.0,
            onevent_est_pct_05_val_365d: 0.0,
            onevent_est_pct_25_val_365d: 0.0,
            onevent_est_pct_75_val_365d: 0.0,
            onevent_est_pct_95_val_365d: 0.0,
            onevent_est_pct_99_val_365d: 0.0,
            updated_at: NaiveDateTime::parse_from_str("2019-03-13 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap(),
        }
    }
}

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

    // csv, msgpack, many others available

    use diesel::prelude::*;
    use diesel::dsl::max;
    use diesel::mysql::MysqlConnection;

    let conn = MysqlConnection::establish("mysql://root@localhost/test")?;
    
    // find last id to avoid inserting duplicate value on official_entity_id col
    let last_entity_id: Option<i32> =
        mkt_cached_alltime_rt_value_stats::table.select(
            max(mkt_cached_alltime_rt_value_stats::official_entity_id))
            .first(&conn)?;

    a.official_entity_id = last_entity_id.unwrap_or(0) + 1;

    diesel::insert_into(mkt_cached_alltime_rt_value_stats::table)
        .values(&a)
        .execute(&conn)?;
    
    // nested, etc.
    use std::collections::HashMap;
    
    #[derive(Debug, Clone, Deserialize, Serialize)]
    enum Things {
        A { one: i32, two: String },
        B { three: HashMap<u64, Vec<bool>>, },
        C
    }

    // array of { string => Things }
    let mut xs: Vec<HashMap<String, Things>> = Default::default();

    let mut kv: HashMap<String, Things> = Default::default();
    kv.insert("key".into(), Things::A { one: 1, two: "value".into() });
    kv.insert("largo".into(), Things::B { three: HashMap::default() });
    xs.push(kv);
    let json = serde_json::to_string_pretty(&xs)?;
    println!("{}", json);
    Ok(())
}
