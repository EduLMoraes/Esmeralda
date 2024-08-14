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
	git commit -m "$(filter-out $@,$(MAKECMDGOALS))"
	git push

release:
	cargo check
	cargo fmt
	make test
	cargo build --release
	cargo bundle --release
	zip -o ./target/release/sources.zip ./src/Views/assets/* ./src/Views/assets/*/* ./src/Views/assets/*/*/* ./src/Views/styles/*
	git add .
	git commit -m "$(filter-out $@,$(MAKECMDGOALS))"
	git push

test:
	cargo test -- --test-threads=1
	make clean

clean:
	rm /tmp/test_db
	rm /tmp/*.csv --force
	rm /tmp/*.pdf
	rm /tmp/*.html
	rm /tmp/non_existent_directory/*
	rm /tmp/nonexistent_directory/*
	rmdir /tmp/non_existent_directory
	rmdir /tmp/nonexistent_directory