import websockets
import asyncio

async def test():
    uri = "wss://websocket-f5e6.onrender.com/ws"
    async with websockets.connect(uri) as ws:
        print(await ws.recv())  # Hello World
        await ws.send("hello")
        print(await ws.recv())  # You said: hello

asyncio.run(test())