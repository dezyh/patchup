FROM node:13.12.0-alpine

WORKDIR /patchup/server
COPY package.json ./
COPY package-lock.json ./
RUN npm install
COPY . ./

EXPOSE 8080
CMD ["npm", "run", "development"]