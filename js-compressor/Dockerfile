# Use Node.js LTS version
FROM node:20-slim

# Set working directory
WORKDIR /usr/src/app

# Copy package files
COPY package*.json ./

# Install dependencies
RUN npm install

# Copy source code
COPY . .

# Make the CLI script executable
RUN chmod +x cli.js

# Set the entrypoint to the CLI
ENTRYPOINT ["node", "cli.js"]
CMD ["--help"] 