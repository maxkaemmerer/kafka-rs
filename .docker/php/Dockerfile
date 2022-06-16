FROM php:8.1-cli

RUN apt update && apt install -y git librdkafka-dev
RUN cd tmp \
    && git clone https://github.com/arnaud-lb/php-rdkafka.git \
    && cd php-rdkafka \
    && phpize \
    && ./configure \
    && make all -j 5 \
    && make install

ADD ./20-kafka.ini /usr/local/etc/php/conf.d/20-kafka.ini

WORKDIR /app