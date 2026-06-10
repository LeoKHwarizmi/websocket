#import websockets
#import asyncio

#async def terminal():
#    uri = "wss://websocket-f5e6.onrender.com/terminal"
#    async with websockets.connect(uri) as ws:
#        print(await ws.recv())  # Terminal ready

#        while True:
#            cmd = input("> ")
#            await ws.send(cmd)
#            print(await ws.recv())

#asyncio.run(terminal())


import websockets
import asyncio

async def terminal():
    uri = "ws://127.0.0.1:10000/terminal"
    async with websockets.connect(uri) as ws:
        print(await ws.recv())  # Terminal ready

        while True:
            cmd = input("> ")
            await ws.send(cmd)
            print(await ws.recv())

asyncio.run(terminal())
