# Bartender


<img src="https://www.flaticon.com/free-icon/bartender_804645?term=bartender&page=1&position=2#" width=300/> 

"Cause they always - got a guy"

Bartender is serverless framework with no containerized runtime that handles REST requests and pass them to the according service.

The servive does whatever that service does - then returns the response to Bartender.

## Example Services

When setting up a service it needs to listen to the incomming requests. This requires a loop that is always listening for incomming messages.

```python
from pynng import Rep0
import json, sys

address = 'tcp://127.0.0.1:13132'

def connect_and_listen():
	with Rep0(listen=address) as rep:
		while True:
		    question = rep.recv()
		    data = json.loads(question.decode("UTF-8"))
		    if "number" in data:
		    	data["number"] = data["number"] + 180
		    	rep.send(json.dumps(data).encode())
		    	print(data)
		    else:
		    	rep.send(json.dumps({}).encode())


if __name__ == '__main__':
    try:
        connect_and_listen()
    except KeyboardInterrupt:
    	sys.exit(0)
```