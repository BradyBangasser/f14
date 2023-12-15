package routes
import (
	"github.com/gin-gonic/gin"
	test_routes_myotherroute "test/routes/myotherroute"
	test_routes_myotherroute_myothersubroute "test/routes/myotherroute/myothersubroute"
	test_routes_myroute "test/routes/myroute"
	test_routes_myroute_mysubroute "test/routes/myroute/mysubroute"
)
func LoadRoutes(router *gin.Engine) {
	router.GET("/", GET)
	router.GET("/myotherroute/", test_routes_myotherroute.GET)
	router.GET("/myotherroute/myothersubroute/", test_routes_myotherroute_myothersubroute.GET)
	router.GET("/myroute/", test_routes_myroute.GET)
	router.GET("/myroute/mysubroute/", test_routes_myroute_mysubroute.GET)
	router.POST("/myroute/", test_routes_myroute.POST)
}