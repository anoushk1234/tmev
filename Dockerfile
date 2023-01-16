FROM library/rust

WORKDIR /usr/src/app

COPY ./searcher-api .

EXPOSE 8080

CMD [ "cargo","run","--release" ] 


