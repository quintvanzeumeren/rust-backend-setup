
install_tools:
	cargo install cargo-udeps \
	cargo install bunyan
	#rustup install nightly

remove_unused_dependencies:
	cargo +nighty udeps

test_with_logs:
	TEST_LOG=true cargo test health_check_works | bunyan

run_and_watch:
	cargo watch -x check -x test -x run | bunyan

#build_dockerfile:
#	cargo sqlx prepare
#	docker build --tag backend --file Dockerfile .

run_dockerfile:
	docker run --rm -p 8000:8000 backend

migrate_db:
	sh scripts/init_db.sh

# Increase the number of open files that can be opened by the system
# use this if you get an error like "Too many open files"
increase_possible_open_files:
	ulimit -n 10000