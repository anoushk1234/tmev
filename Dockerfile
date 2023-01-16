FROM library/rust

WORKDIR /usr/src/app

COPY ./searcher-api .

CMD [ "cargo","run","--release" ] 

EXPOSE 8080
