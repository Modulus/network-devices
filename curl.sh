#!/bin/bash
curl -H "Accept: application/json" -d '{ "name": "apple.tv", "domain": "local", "address" : "10.0.0.100", "icon" : "NaN", "comment": "ojda"}' -X POST http://localhost:3000/device
curl -X GET http://localhost:3000/device
