//! 0x0B. Databases. `model` is given; the tasks are the `t*` modules. The schema is in
//! `migrations/0001_init.sql`.

pub mod model;

pub mod t00_insert_employee;
pub mod t01_record_holding;
pub mod t02_expiring_between;
pub mod t03_transfer_employee;
pub mod t04_engine_bridge;
pub mod t100_import_employees;
