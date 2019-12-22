from pynng import Rep0
import json

def connect_and_listen(patron_handler, address):
	with Rep0(listen=address) as rep:
		while True:
			question = rep.recv()
			data = json.loads(question.decode("UTF-8"))
			to_send_bytes_ = patron_handler({}, data)
			rep.send(to_send_bytes_.encode())

class Bartender(object):
	"""docstring for Bartender"""
	def __init__(self):
		pass

	def start(function, address):
		connect_and_listen(function, address)