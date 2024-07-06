import socket
import json

def send_request(action, token, amount=None):
    with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as client:
        client.connect("/tmp/rust_ipc_wallet.sock")
        request = {"action": action, "token": token}
        if amount is not None:
            request["amount"] = amount
        client.sendall(json.dumps(request).encode())
        response = client.recv(1024)
        return json.loads(response)

print(send_request("add", "token1", 50))
print(send_request("subtract", "token1", 30))
print(send_request("balance", "token1"))
