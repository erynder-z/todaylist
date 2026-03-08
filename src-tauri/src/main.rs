//! Main entry point for the todaynote application.
//!
//! This is a thin wrapper around the `todaynote_lib::run()` function
//! which handles the application's core logic.

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    todaynote_lib::run()
}
