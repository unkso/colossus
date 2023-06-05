#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::use_self)]

//! The Domain model and types for UNKSO Colossus

/// Module containing everything regarding model repositories
pub mod repositories;

/// Module containing types relating to users
pub mod user_types;

/// Module containing `SeaOrm` entities
pub mod entity;
