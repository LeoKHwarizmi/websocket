import websockets, asyncio

async def test():
    ws = await websockets.connect("wss://your-service.onrender.com/ws")
    print(await ws.recv())

asyncio.run(test())
