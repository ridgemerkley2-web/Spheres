"""Disposable self-owned fault: never accepts a target PID or external process."""
import ctypes
import os

k = ctypes.WinDLL("kernel32", use_last_error=True)
k.SetErrorMode(0x0001 | 0x0002)
k.CreateThread.argtypes = [ctypes.c_void_p, ctypes.c_size_t, ctypes.c_void_p,
                          ctypes.c_void_p, ctypes.c_ulong, ctypes.POINTER(ctypes.c_ulong)]
k.CreateThread.restype = ctypes.c_void_p
k.WaitForSingleObject.argtypes = [ctypes.c_void_p, ctypes.c_ulong]
thread_id = ctypes.c_ulong()
print(f"owned_fault_pid={os.getpid()}", flush=True)
# A new thread in THIS disposable interpreter attempts to execute address 1.
# Unlike a fault inside a ctypes call, this cannot be converted to a Python error.
handle = k.CreateThread(None, 0, ctypes.c_void_p(1), None, 0, ctypes.byref(thread_id))
if not handle:
    raise ctypes.WinError(ctypes.get_last_error())
print(f"owned_fault_thread={thread_id.value}", flush=True)
k.WaitForSingleObject(handle, 10_000)
raise RuntimeError("The deliberately invalid thread unexpectedly survived")
