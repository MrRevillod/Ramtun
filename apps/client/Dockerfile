FROM node:24-alpine
WORKDIR /app

RUN apk add --no-cache curl

COPY ./apps/client/package*.json ./

RUN npm install

EXPOSE 5173

CMD ["npm", "run", "dev"]
