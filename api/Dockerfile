FROM rust:1.60
WORKDIR /usr/src
COPY . .
RUN cargo build
ENV PORT=8080
EXPOSE ${PORT}
CMD [ "/usr/src/target/debug/api" ]