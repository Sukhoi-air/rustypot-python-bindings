import rustypot
import time
import numpy as np


# io = rustypot.feetech("/dev/ttyACM0", 1000000)
controller = rustypot.FeetechController("/dev/ttyACM0", 1000000, 500, [1], [32])
controller.set_new_target([0])  
time.sleep(2)

update_freq = 50
F = 1
A = 90
try:
    while True:
        new_target = A*np.sin(2*np.pi*F*time.time())
        controller.set_new_target([new_target])  
        print(controller.get_present_position())
        print(controller.get_current_speed())
        time.sleep(1/update_freq)
except KeyboardInterrupt:
    controller.freeze()
    # controller.disable_torque()
    # controller.set_new_target([0])  
    time.sleep(1)