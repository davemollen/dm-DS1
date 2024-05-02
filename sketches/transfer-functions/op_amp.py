from scipy import signal
import numpy as np
import matplotlib.pyplot as plt
from enum import Enum

# Set the sample rate
sample_rate = 44100  # in Hz

# Change the dist value to see the difference in the frequency response
# Keep it between 0 and 1
dist = 1.

def get_z_domain_coefficients(dist):
  double_sr = sample_rate * 2
  squared_double_sr = pow(double_sr, 2)

  dist = max(dist, 0.00001)

  rt = dist * 100000.
  rb = ((1. - dist) * 100000.) + 4700.
  # cz = 0.00000047
  # cc = 0.0000000001
  cz = 0.000001
  cc = 0.00000000025

  a = 1. / (rt * rb * cz * cc)
  c = 1. / (rt * cc) + 1. / (rb * cz)
  b = c + 1. / (rb * cc)

  a0 = a + c * double_sr + squared_double_sr
  b0 = (a + b * double_sr + squared_double_sr) / a0
  b1 = (2. * a - 2. * squared_double_sr) / a0
  b2 = (a - b * double_sr + squared_double_sr) / a0
  a1 = b1
  a2 = (a - c * double_sr + squared_double_sr) / a0

  return ([b0, b1, b2], [1, a1, a2])

# Get z-domain coefficients
b, a = get_z_domain_coefficients(dist)
print('z-domain coefficients', (list(b), list(a)))

# Get the frequency response
w,h = signal.freqz(b, a, 2**20)
w = w * sample_rate / (2 *np.pi)

# Plot the frequency response
fig1 = plt.figure(1)
plt.title('Digital filter frequency response')
plt.semilogx(w, 20 * np.log10(abs(h)), 'b')
plt.ylabel('magnitude [dB]')
plt.xlabel('frequency [Hz]')
plt.grid()
plt.axis('tight')
plt.xlim([10, 20000])
plt.ylim([0, 30])
plt.show()