mod common;

use async_backend::app::routes;
use axum::http::StatusCode;
use common::{call, state};

async fn numbers(uri: &str) -> Vec<u64> {
    let answer = call(routes(state()), "GET", uri, None, None).await;
    assert_eq!(answer.status, StatusCode::OK, "{uri}: {}", answer.text);
    answer
        .json()
        .as_array()
        .expect("a list")
        .iter()
        .map(|e| e["staff_number"].as_u64().expect("a staff number"))
        .collect()
}

#[tokio::test]
async fn lists_everyone_in_staff_number_order() {
    assert_eq!(numbers("/employees").await, [10007, 20020, 30303, 48213]);
}

#[tokio::test]
async fn filters_by_station() {
    assert_eq!(numbers("/employees?station=NBO").await, [20020, 48213]);
    assert!(numbers("/employees?station=EDL").await.is_empty());
}

#[tokio::test]
async fn pages_with_after_and_limit() {
    assert_eq!(numbers("/employees?limit=2").await, [10007, 20020]);
    assert_eq!(
        numbers("/employees?limit=2&after=20020").await,
        [30303, 48213]
    );
    assert!(numbers("/employees?limit=2&after=48213").await.is_empty());
}

#[tokio::test]
async fn station_filter_and_paging_together() {
    assert_eq!(numbers("/employees?station=NBO&after=20020").await, [48213]);
}

#[tokio::test]
async fn a_limit_of_zero_is_refused_and_a_huge_one_is_capped() {
    let zero = call(routes(state()), "GET", "/employees?limit=0", None, None).await;
    assert_eq!(zero.status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(numbers("/employees?limit=100000").await.len(), 4);
}
