import asyncio
import ctypes

libc = ctypes.CDLL('libc.so.6')

async def blocking_io_loop(num):
    print(f'Entering blocking IO {num}...')
    # libc.usleep(1000000)
    await asyncio.sleep(1)
    print(f'Leaving blocking IO {num}...')

async def main():
    tasks = []
    for i in range(5):
        task = asyncio.create_task(blocking_io_loop(i))
        tasks.append(task)
    for t in tasks:
        await t

asyncio.run(main())
print('Done')