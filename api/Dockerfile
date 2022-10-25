FROM node:latest
WORKDIR /usr/src
COPY . .
RUN ["yarn"]
ENV PORT=8080
EXPOSE ${PORT}
CMD [ "node", "/usr/src" ]