use clap::{Parser, Subcommand};

#[derive(Debug,Clone,Subcommand)]
pub enum Action {
    Start
}

#[derive(Debug,Clone,Parser)]
pub struct Arg {
    #[clap(subcommand)]
    pub action: Action
}