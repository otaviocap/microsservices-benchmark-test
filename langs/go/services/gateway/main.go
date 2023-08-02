package main

import (
	"go-gateway/server"

	"github.com/gin-gonic/gin"
)

func main() {
	app := gin.Default()

	routes, _ := server.GetRoutes()

	server.RegisterReverseProxy(routes, app)

	app.Run()
}
