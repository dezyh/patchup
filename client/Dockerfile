FROM node:13.12.0-alpine

WORKDIR /patchup/client
COPY package.json ./
COPY package-lock.json ./
RUN npm install
COPY . ./

EXPOSE 3000
CMD ["npm", "start"]
