config:
	sudo apt install gdk3.0

run:
	clear
	cargo build
	sudo ./target/debug/esmeralda > log.txt

git:
	git add .
	git commit -m "commited with makefile"