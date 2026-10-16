//! 0x0C. The TWA server. `model` and `app` are given; the tasks are the `t*` modules.
//! Do task 1 before the others: every error response goes through it.

pub mod app;
pub mod model;

pub mod t00_async_basics;
pub mod t01_api_error;
pub mod t02_employees;
pub mod t03_transfers;
pub mod t04_auth;
pub mod t100_list;
