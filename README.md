# Bartender

### Oh yea? I got a guy..

<img src="https://image.flaticon.com/icons/png/512/804/804645.png" width=200/> 


Bartender is microservce framework with no containerized runtime that handles REST requests and pass them to the according service.

The servive does whatever that service does - then returns the response to Bartender. As of now we only support Python functions - but Golang, and Rust are next.

## Product Goals

Allow developers to build microservice apps by providing a premade REST gateway that any service in any language can be attached to and response in near zero latency.

```
 REST        Microservices

      +----+
      |    +-------->
      |    |
+---->+    +-------->
      |    |
      |    +-------->
      |    |
      +----+

```

1. Allow developers to build polyglot applications
2. Give developers an easy way to build microservice apps
3. Should allow for frictionless attachment to gateway
4. Allow functions to have long running processe
5. Almost 0 latency relaying data to funcion


## Creating and attaching a service

When setting up a service it needs to listen to the incomming requests. This requires a loop that is always listening for incomming messages.

```python
import json

def patron_handler(context, event):
	print("Hi, I'm a microservice!")
	return json.dumps(event)


# to attach the process
# just import Bartender and
# pass the function and an 
# unused address.

from bartender import Bartender

Bartender.start(
	function = patron_handler,
	address = 'tcp://127.0.0.1:13131'
)
```


### Adding and calling services

Registering a service
```http
POST /config HTTP/1.1
Content-Type: application/json
Host: localhost:8080
Content-Length: 56

{
	"key": "SERVICE_A",
	"loc": "tcp://127.0.0.1:13131"
}
```

Viewing registered services
```http
GET /list HTTP/1.1
Host: localhost:8080
```

Executing a service
```http
POST / HTTP/1.1
Content-Type: application/json
Host: localhost:8080
Content-Length: 43

{
	"service": "SERVICE_A",
	"number": 208
}
```

### Bartender Multiplexing

Bartender can also run the services for you on your local machine, this is achieved by creating a `tmux` session and executing the program as an individual program. 

This way you can freely develop and attach functions - but when you have settled on a build, you let Bartender handle the runtime and essentially deploy the function to Bartender. 

This functionality is only for development since it is limited to running tmux. For a distribut
ed system you'd need to manage your docker containers and the networking between boxes.

```http
GET /init HTTP/1.1
Host: localhost:8080
```

```http
GET /terminate HTTP/1.1
Host: localhost:8080
```

### Stopping 

Press. 
`control-Z`. 
then
```bash
kill %1
```

## Dependencies

```bash
sudo apt install tmux
```

```bash
pip3 install pynng
```