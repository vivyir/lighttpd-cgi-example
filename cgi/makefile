all:
	cargo build --release
	cp target/release/cgi ../www/cgi
	cargo clean
	lighttpd -Df ../lighttpd.conf
