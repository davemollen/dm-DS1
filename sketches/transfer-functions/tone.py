from scipy import signal
import numpy as np
import matplotlib.pyplot as plt

# Set the sample rate
sample_rate = 44100  # in Hz

# Change the tone value to see the difference in the frequency response
# Keep it between 0 and 1
tone = 0.5

# Set cutoff frequencies
low_pass_cutoff = 234.05138689985
high_pass_cutoff = 1063.8699404538

# Apply Butterworth filters
b_low, a_low = signal.butter(1, low_pass_cutoff, 'low', fs=sample_rate)
b_high, a_high = signal.butter(1, high_pass_cutoff, 'high', fs=sample_rate)

# Get the frequency response
w_low, h_low = signal.freqz(b_low, a_low, 2**20)
w_high, h_high = signal.freqz(b_high, a_high, 2**20)

# Apply gains to the frequency response of lowpass and highpass filters
h_low *= ((1. - tone) * 0.595235 + 0.202379)
h_high *= (tone * 0.694642 + 0.002896)

# Combine the frequency responses
h_combined = h_low + h_high

# Convert frequency to Hz
frequencies = (w_low / (2 * np.pi)) * sample_rate

# Plot the frequency response
fig = plt.figure()
plt.title('Digital filter frequency response')
plt.semilogx(frequencies, 20 * np.log10(abs(h_combined)), 'b')
plt.ylabel('magnitude [dB]')
plt.xlabel('frequency [Hz]')
plt.grid()
plt.axis('tight')
plt.xlim([10, 20000])
plt.ylim([-70, 0])
plt.show()