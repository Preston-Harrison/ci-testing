FROM rust:1.60
WORKDIR /usr/src/app
COPY . .
RUN ["cargo", "build"]
EXPOSE ${PORT}
CMD [ "/usr/src/app/target/debug/api" ]