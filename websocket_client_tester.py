import asyncio
import websockets


async def connect():
    async with websockets.connect("ws://127.0.0.1:8080") as websocket:
        await websocket.send("Hello, world!")
        while True:
            response = await websocket.recv()
            print("Server: ")

            print(response)
            print('\n')

asyncio.run(connect())