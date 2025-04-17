# Use Node.js LTS version
FROM node:20-slim

# Set working directory
WORKDIR /usr/src/js-compressor

# Copy package files
COPY package*.json ./

# Install dependencies
RUN npm install

# Copy source code
COPY . .

# Make the CLI executable
RUN chmod +x index.js

# Set the entrypoint
ENTRYPOINT ["node", "index.js"]
CMD ["--help"] 