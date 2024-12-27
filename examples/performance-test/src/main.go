package main

import (
    "github.com/optimizely/go-sdk/pkg/client"
    "github.com/optimizely/go-sdk/pkg/decide"
    "github.com/optimizely/go-sdk/pkg/logging"
    "os"
    "fmt"
)

func main() {
    logging.SetLogLevel(logging.LogLevelError)

    datafile, _ := os.ReadFile("../../datafiles/sandbox.json")

    optimizelyFactory := &client.OptimizelyFactory{
        Datafile: datafile,
    }
    optimizelyClient, _ := optimizelyFactory.Client()

    flag_key := "buy_button"
    attributes := map[string]interface{}{}
    options := []decide.OptimizelyDecideOptions{decide.DisableDecisionEvent}

    for i := 0; i < 1000000; i++ {
        userId := fmt.Sprintf("user%d", i)
        userContext := optimizelyClient.CreateUserContext(userId, attributes)

        decision := userContext.Decide(flag_key, options)

        _ = decision
    }
}