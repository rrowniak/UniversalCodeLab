from threading import Thread
import ctypes

libc = ctypes.CDLL('libc.so.6')

def blocking_io_loop(num):
    print(f'Entering blocking IO {num}...')
    libc.usleep(1000000)
    print(f'Leaving blocking IO {num}...')


for i in range(5):
    thread = Thread(target=blocking_io_loop, args=[i])
    thread.run()
#blocking_io_loop(1)