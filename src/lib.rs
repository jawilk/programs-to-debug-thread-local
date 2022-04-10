use std::cell::RefCell;

use solana_sdk::pubkey::Pubkey;

thread_local! {
    pub static PROGRAMS_TO_DEBUG: RefCell<Vec<Pubkey>> = RefCell::new(vec![]);
}

pub fn set_thread_local(pubkeys: &[&Pubkey]) -> () {
    PROGRAMS_TO_DEBUG.with(|refcell| {
        let mut debug_programs = refcell.borrow_mut();
        for i in 0..pubkeys.len() {
            debug_programs.push(*pubkeys[i]);
        }
    })
}

pub fn is_debug_progam(pubkey: &Pubkey) -> bool {
    PROGRAMS_TO_DEBUG.with(|refcell| {
        if refcell.borrow().contains(pubkey) {
            return true;
        }
        false
    })
}
