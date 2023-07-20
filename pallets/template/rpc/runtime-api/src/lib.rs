#![cfg_attr(not(feature = "std"), no_std)]

// Here we declare the runtime API. It is implemented it the `impl` block in
// runtime file (the `runtime/src/lib.rs`)
sp_api::decl_runtime_apis! {
	pub trait TemplateApi {
		fn sum_5() -> u32;
		fn sum_and_store(value: u32) -> u32;
	}
}
