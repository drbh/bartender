from pynng import Req0, Rep0
import json, sys

# address = 'inproc://nng/example'
address = 'tcp://127.0.0.1:13131'

# with Rep0(listen=address) as rep, Req0(dial=address) as req:
#     req.send(b'random.random()')
#     question = rep.recv()
#     answer = b'4'  # guaranteed to be random
#     rep.send(answer)
#     print(req.recv())  # prints b'4'

def connect_and_listen():
	with Rep0(listen=address) as rep:
		while True:
		    question = rep.recv()
		    data = json.loads(question.decode("UTF-8"))
		    data["number"] = data["number"] + 180
		    rep.send(json.dumps(data).encode())
		    print(data)


if __name__ == '__main__':
    try:
        connect_and_listen()
    except KeyboardInterrupt:
    	sys.exit(0)
