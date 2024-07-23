from scipy import signal
import numpy as np
import matplotlib.pyplot as plt
from enum import Enum

# Set the sample rate
sample_rate = 44100  # in Hz

# Change the tone value to see the difference in the frequency response
# Keep it between 0 and 1
tone = 0.75

def get_s_domain_coefficients(t):
    # The following transfer function was derived with QsapecNG:
    # (1-t)*R3 * C1 * C2 * R2 * R4 * V1 * s^2 + ( C1 * R4 * V1 * t*R3 + (1-t)*R3 * C1 * R4 * V1 + C1 * R2 * R4 * V1 + C1 * R1 * R4 * V1 + C1 * R1 * V1 * t*R3 ) * s + ( R4 * V1 + V1 * t*R3 )
    # ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
    # ( C1 * C2 * R2 * R4 * t*R3 + (1-t)*R3 * C1 * C2 * R2 * R4 + C1 * C2 * R1 * R2 * R4 + C1 * C2 * R1 * R2 * t*R3 + (1-t)*R3 * C1 * C2 * R1 * R2 ) * s^2 + ( C1 * R4 * t*R3 + (1-t)*R3 * C1 * R4 + C1 * R2 * R4 + C1 * R1 * R4 + C1 * R1 * t*R3 + (1-t)*R3 * C1 * R1 + C1 * R1 * R2 + C2 * R2 * R4 + C2 * R2 * t*R3 + (1-t)*R3 * C2 * R2 ) * s + ( R4 + t*R3 + (1-t)*R3 + R2 )

    # This function implements this transfer function, but with less repeated calculations.

    R1 = 2200
    R2 = 6800
    R3 = 20000
    R4 = 6800
    C1 = 2.2e-8
    C2 = 1e-7

    R3_a = t * R3
    R3_b = (1-t) * R3

    C1C2 = C1 * C2
    R2R4 = R2 * R4
    C1C2R2R4 = C1C2 * R2R4
    C1C2R1R2 = C1C2 * R1 * R2
    C1R4 = C1 * R4
    C1R1 = C1 * R1
    C2R2 = C2 * R2

    b0 = C1C2R2R4 * R3_a
    b1 = C1R4 * R3_b + C1R4 * R3_a + C1 * R2R4 + C1R1 * R4 + C1R1 * R3_b
    b2 = R4 + R3_b
    a0 = b0 + C1C2R2R4 * R3_b + C1C2R2R4 * R1 + C1C2R1R2 * R3_b + C1C2R1R2 * R3_a
    a1 = b1 + C1R1 * R3_a + C1R1 * R2 + C2R2 * R4 + C2R2 * R3_b + C2R2 * R3_a
    a2 = R4 + R3_b + R3_a + R2

    return ([b0, b1, b2], [a0, a1, a2])

# Get s-domain coefficients
num, den = get_s_domain_coefficients(tone)
print('s-domain coefficients', (num, den))

# Apply bilinear transform
b, a = signal.bilinear(num, den, fs=sample_rate)
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
plt.ylim([-40, 0])
plt.show()