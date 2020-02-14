FROM alpine:latest

WORKDIR /temp
RUN wget 'https://github.com/rexx0520/bb/releases/latest/download/x86_64-unknown-linux-musl.zip' && unzip x86_64-unknown-linux-musl.zip && mkdir /app

RUN cp ./x86_64-unknown-linux-musl/release/bb /app/

WORKDIR /app
RUN rm -rf /temp
RUN chmod +x ./bb

CMD /app/bb