run:
	clear
	cargo build
	sudo ./target/debug/esmeralda

git:
	git add .
	git commit -m "commited with makefile"