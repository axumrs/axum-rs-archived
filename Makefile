pkg: build
	mkdir axum-rs && \
	cp ./target/release/axum-rs ./axum-rs/ && \
	cp -r ./templates ./axum-rs/ && \
	cp -r ./static ./axum-rs/ && \
	cp ./env.example ./axum-rs/.env && \
	tar zcvf ./axum-rs.tar.gz ./axum-rs
build: 
	cargo build --release

clear:
	rm -rf ./axum-rs.tar.gz && \
	rm -rf ./axum-rs
