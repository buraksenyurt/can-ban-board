ARG NODE_VERSION=20.13.1

FROM node:${NODE_VERSION}-alpine

# Use production node environment by default.
ENV NODE_ENV production

WORKDIR /usr/src/app

COPY frontend/package.json .
RUN corepack enable
RUN yarn set version stable
RUN yarn install

# Copy the rest of the source files into the image.
COPY frontend/public public
COPY frontend/server.js .

# Expose the port that the application listens on.
EXPOSE 4447

# Run the application.
CMD yarn start
