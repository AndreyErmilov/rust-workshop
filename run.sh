#!/bin/bash

for i in $(seq 1 1 4)
do
  curl -X POST localhost:8080/v1/events/ -H "content-type: application/json" -d @test-data/0"$i"-send-message.json
done