FROM rust:1.23.0
MAINTAINER Francois-Guillaume Ribreau <docker@fgribreau.com>

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install

CMD ["rss-to-lametric"]
