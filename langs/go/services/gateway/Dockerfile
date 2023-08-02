FROM golang:1.20

WORKDIR /app
COPY . .

# Download all the dependencies
RUN go get -d -v ./...

# Install the package
RUN go install -v ./...

# Run the executable
CMD ["go-gateway"]