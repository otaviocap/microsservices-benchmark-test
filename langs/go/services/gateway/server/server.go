package server

import (
	"bufio"
	"log"
	"net/http"
	"net/url"
	"strings"

	"github.com/gin-gonic/gin"
	"golang.org/x/exp/slices"
)

// Reference: https://github.com/gin-gonic/examples/blob/master/reverse-proxy/reverseServer/main.go
func RegisterReverseProxy(routes []Route, app *gin.Engine) {

	app.NoRoute(func(c *gin.Context) {
		req := c.Request

		microserviceNameFromPath := strings.Split(c.Request.URL.Path, "/")[1]

		routeIdx := slices.IndexFunc(routes, func(r Route) bool { return r.Name == microserviceNameFromPath })

		if routeIdx == -1 {
			log.Printf("Could not find route: %s\n", microserviceNameFromPath)

			return
		}

		microservice := routes[routeIdx]
		redirect, err := url.Parse(microservice.Redirect)

		if err != nil {
			log.Printf("Error getting URL from routes configuration. Route: %s\n", microserviceNameFromPath)
			return
		}

		req.URL.Scheme = redirect.Scheme
		req.URL.Host = redirect.Host
		req.URL.Path = strings.Join(strings.Split(c.Request.URL.Path, "/")[2:], "/")

		transport := http.DefaultTransport

		log.Printf("Handling request to %s", req.URL)

		resp, err := transport.RoundTrip(req)

		if err != nil {
			log.Printf("Error in proxying to microservice (%s / [%s])\n", microservice.Name, microservice.Redirect)
			return
		}

		for key, info := range resp.Header {
			for _, value := range info {
				c.Header(key, value)
			}
		}

		defer resp.Body.Close()
		bufio.NewReader(resp.Body).WriteTo(c.Writer)
	})

}
