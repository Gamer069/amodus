#[macro_export]
macro_rules! eh {
	($res:ident, $prefix:expr) => {
		let $res = match $res {
			Ok(val) => val,
			Err(err) => {
				eprintln!("{}: {}", $prefix, err);
				std::process::exit(-1);
			}
		};
	};
}

#[macro_export]
macro_rules! eh_mut {
	($res:ident, $prefix:expr) => {
		let mut $res = match $res {
			Ok(val) => val,
			Err(err) => {
				eprintln!("{}: {}", $prefix, err);
				std::process::exit(-1);
			}
		};
	};
}

#[macro_export]
macro_rules! eh_direct {
	($res:expr, $prefix:expr) => {
		if let Err(err) = $res {
			eprintln!("{}: {}", $prefix, err);
			std::process::exit(-1);
		};
	};
}

#[macro_export]
macro_rules! vlog {
	($verbose:expr, $($arg:tt)*) => {
		if $verbose {
			println!($($arg)*);
		}
	};
}

#[macro_export]
macro_rules! vlog_no {
	($verbose:expr, $($arg:tt)*) => {
		if $verbose {
			print!($($arg)*);
			let _ = std::io::stdout().flush();
		}
	};
}

#[macro_export]
macro_rules! log {
	($($arg:tt)*) => {
		println!($($arg)*);
	};
}

#[macro_export]
macro_rules! log_no {
	($($arg:tt)*) => {
		print!($($arg)*);
		let _ = std::io::stdout().flush();
	};
}
