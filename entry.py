import websockets
import asyncio

async def test():
    #uri = "wss://websocket-f5e6.onrender.com/ws"
    uri = "wss://websocket-f5e6.onrender.com/terminal"
    #uri = "ws://127.0.0.1:10000/ws"
    #uri = "ws://127.0.0.1:10000/terminal"
    async with websockets.connect(uri) as ws:
        print(await ws.recv())  # Hello World
        await ws.send("hello")
        print(await ws.recv())  # You said: hello

asyncio.run(test())