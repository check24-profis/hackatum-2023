ARG SERVER_PORT

FROM node:18-slim

WORKDIR /usr/src

COPY package*.json ./

RUN npm install

COPY . .

CMD ["node", "index.js"]