#![no_std]
extern crate alloc;

#[macro_use]
extern crate playdate as pd;

use core::ptr::NonNull;
use fixedbitset::FixedBitSet;
use pd::sys::EventLoopCtrl;
use pd::sys::ffi::PlaydateAPI;
use pd::system::prelude::*;

/// Entry point
#[no_mangle]
pub fn event_handler(_api: NonNull<PlaydateAPI>, _event: SystemEvent, _sim_key_code: u32) -> EventLoopCtrl {
	println!("before call");
	// seg fault occurs here
	let bitset = FixedBitSet::with_capacity(2);
	// print out so the above doesn't get optimized out
	println!("{}", bitset[0]);
	
	EventLoopCtrl::Continue
}


// Needed for debug build, absolutely optional
ll_symbols!();
