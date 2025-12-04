FROM node:20.12.0

RUN corepack enable && corepack prepare yarn@4.1.1 --activate
RUN yarn --version

WORKDIR /app

COPY package.json yarn.lock ./

RUN yarn config set nodeLinker node-modules

RUN yarn install && yarn cache clean

COPY . .

RUN yarn build

EXPOSE 3000

CMD ["yarn", "start"]