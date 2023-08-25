FROM node:lts AS builder

WORKDIR /opt/cowsite
COPY package.json package.json
COPY package-lock.json package-lock.json
RUN npm i
COPY . .

FROM builder AS production

CMD npm run start
