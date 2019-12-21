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