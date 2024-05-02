from scipy import signal
import numpy as np
import matplotlib.pyplot as plt
from enum import Enum

# Set the sample rate
sample_rate = 44100  # in Hz

gain = 1.

def get_z_domain_coefficients(gain):
  double_sample_rate = 2. * sample_rate
  sample_period = 1. / double_sample_rate

  freq1 = 3.077643
  # freq2 = 703.162476
  freq2 = 600
  gain = 63.095734

  radians1 = np.tan(freq1 * np.pi * 2. * sample_period) * double_sample_rate
  radians2 = np.tan(freq2 * np.pi * 2. * sample_period) * double_sample_rate

  a = radians1 + double_sample_rate
  b = radians1 - double_sample_rate
  c = radians2 + double_sample_rate
  d = radians2 - double_sample_rate

  a0 = a * c
  a1 = (a * d + b * c) / a0
  a2 = b * d / a0
  b0 = double_sample_rate * double_sample_rate / a0 * gain
  b1 = b0 * -2.
  b2 = b0

  return ([b0, b1, b2], [1., a1, a2])

# Get z-domain coefficients
b, a = get_z_domain_coefficients(gain)
print('z-domain coefficients', (list(b), list(a)))

# Get the frequency response
w,h = signal.freqz(b, a, 2**20)
w = w * sample_rate / (2 *np.pi)
print(w)
print(h)

# Plot the frequency response
fig1 = plt.figure(1)
plt.title('Digital filter frequency response')
plt.semilogx(w, 20 * np.log10(abs(h)), 'b')
plt.ylabel('magnitude [dB]')
plt.xlabel('frequency [Hz]')
plt.grid()
plt.axis('tight')
plt.xlim([1, 20000])
plt.ylim([-40, 40])
plt.show()