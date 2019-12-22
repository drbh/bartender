import json

def patron_handler(context, event):
	print(event)
	return json.dumps(event)

from bartender import Bartender

Bartender.start(
	function = patron_handler,
	address = 'tcp://127.0.0.1:13131'
)