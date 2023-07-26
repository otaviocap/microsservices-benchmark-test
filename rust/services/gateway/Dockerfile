FROM rust:1.67

WORKDIR /usr/src/rust-microsservice
COPY . .

RUN cargo install --path .

CMD ["gateway"]