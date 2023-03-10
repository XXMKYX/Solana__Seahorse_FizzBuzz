#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use crate::{id, seahorse_util::*};
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use std::{cell::RefCell, rc::Rc};
pub fn huevos_handler<'info>(mut payer: SeahorseSigner<'info, '_>) -> () {
    solana_program::msg!("{}", "huevos".to_string());
}
