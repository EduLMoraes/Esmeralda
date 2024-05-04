config:
	sudo apt install gdk3.0

run:
	clear
	cargo build
	sudo ./target/debug/esmeralda > log.txt

git:
	cargo check
	cargo fmt
	make test
	git add .
	git commit -m "commited with makefile"
	git push

test:
	clear
	echo testando...
	cargo test -- --test-threads=1
	echo limpando...
	rm /tmp/*.csv --force
	rm /tmp/*.pdf
	rm /tmp/*.html
	rm /tmp/test_db
	rm /tmp/non_existent_directory/*
	rm /tmp/nonexistent_directory/*
	rmdir /tmp/non_existent_directory
	rmdir /tmp/nonexistent_directory
	echo testes finalizados
