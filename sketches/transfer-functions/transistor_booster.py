from scipy import signal
import numpy as np
import matplotlib.pyplot as plt
from enum import Enum

# Set the sample rate
sample_rate = 44100  # in Hz

# This is the gain of the transistor amp
gain = 63.095734

def get_s_domain_coefficients():
  freq1 = 3.3862753849339
  freq2 = 673.449
  w1 = np.pi * 2. * freq1
  w2 = np.pi * 2. * freq2

  num = [1, 0, 0]
  den = [1, w1+w2, w1*w2]
  return (num, den)

# Get s-domain coefficients
num, den = get_s_domain_coefficients()
print('s-domain coefficients', (num, den))

# Apply bilinear transform
b, a = signal.bilinear(num, den, fs=sample_rate)
print('z-domain coefficients', (list(b), list(a)))

# Apply gain
b = [item * gain for item in b]

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
plt.xlim([1, 20000])
plt.ylim([-40, 40])
plt.show()