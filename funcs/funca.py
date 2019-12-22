from pynng import Rep0
import json, sys

address = 'tcp://127.0.0.1:13131'


def patron_handler(context, event):
	print("hello world")
	return json.dumps(event)



def connect_and_listen():
	with Rep0(listen=address) as rep:
		while True:
			question = rep.recv()
			data = json.loads(question.decode("UTF-8"))
			to_send_bytes_ = patron_handler({}, data)
			rep.send(to_send_bytes_.encode())


if __name__ == '__main__':
	try:
		connect_and_listen()
	except KeyboardInterrupt:
		sys.exit(0)