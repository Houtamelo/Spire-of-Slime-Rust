use crate::*;

pub(in super) fn init_panic_hook() {
	let old_hook = std::panic::take_hook();
	std::panic::set_hook(Box::new(move |panic_info| {
		let loc_string;
		if let Some(location) = panic_info.location() {
			loc_string = format!("file '{}' at line {}", location.file(), location.line());
		} else {
			loc_string = own!("unknown location")
		}

		let error_message;
		if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
			error_message = format!("[RUST] {}: panic occurred: {:?}", loc_string, s);
		} else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
			error_message = format!("[RUST] {}: panic occurred: {:?}", loc_string, s);
		} else {
			error_message = format!("[RUST] {}: unknown panic occurred", loc_string);
		}
		godot_error!("{}", error_message);
		(*(old_hook.as_ref()))(panic_info);

		unsafe {
			if let Some(gd_panic_hook) = autoload::<Node>("rust_panic_hook") {
				gd_panic_hook.call("rust_panic_hook", &[GodotString::from_str(error_message).to_variant()]);
			}
		}
	}));
}