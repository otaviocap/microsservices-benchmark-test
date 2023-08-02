package server

import (
	"encoding/json"
	"log"
	"os"
)

type Route struct {
	Name     string `json:"name"`
	Redirect string `json:"redirect"`
}

func GetRoutes() ([]Route, error) {

	routesFile, err := os.ReadFile("./routes.json")

	if err != nil {
		log.Println("Error while reading routes configuration file")

		return nil, err
	}

	var routes []Route

	json.Unmarshal(routesFile, &routes)

	return routes, nil

}
