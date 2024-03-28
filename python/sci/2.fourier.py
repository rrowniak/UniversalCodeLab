import matplotlib.pyplot as plt
import numpy as np
import scipy.integrate as integrate


# define functions

# Parameters
amplitude = 1.0
frequency = 2.0  # Frequency of the sine wave
phase_shift = np.radians(30)
noise_amplitude = 0.1

# Generate time values
t = np.linspace(0, 2*np.pi, 1000)

# Generate the clean sine wave
clean_sine_wave = amplitude * np.sin(frequency * t + phase_shift)

# Add noise
noisy_sine_wave = clean_sine_wave + \
    noise_amplitude * np.random.normal(size=len(t))

# Fourier analysis


class FourierSeries:
    def __init__(self, fn, T, N):
        self.fn = fn
        self.w = 2 * np.pi / T
        self.T = T
        self.N = N + 1
        self.A = []
        self.B = []

        self._cals_coefficients()

    def _cals_coefficients(self):
        T = self.T
        w = self.w

        def fa(t, n):
            return self.fn(t) * np.cos(n * w * t)

        def fb(t, n):
            return self.fn(t) * np.sin(n * w * t)

        for n in range(0, self.N):
            # Calc A[n]
            def f(t): return fa(t, n)
            a = 2 * integrate.quad(f, -T / 2, T / 2)[0] / T
            self.A.append(a)

            # calc B[n]
            if n == 0:
                self.B.append(0.0)
                continue

            def f(t): return fb(t, n)
            b = 2 * integrate.quad(f, -T / 2, T / 2)[0] / T
            self.B.append(b)

    def __call__(self, t):
        s = self.A[0] / 2
        for n in range(1, self.N):

            w = self.w
            s += self.A[n] * np.cos(n * w * t)
            s += self.B[n] * np.sin(n * w * t)
        return s


def my_sin(t):
    # return np.sin(t * 2)
    if np.sin(t) > 0:
        return 1
    else:
        return -1


fs = FourierSeries(my_sin, 2 * np.pi, 5)
fs_arr = fs(t)


# Plotting
plt.figure(figsize=(8, 6))
plt.plot(t, amplitude * np.sin(frequency * t), label='Original signal')
plt.plot(t, clean_sine_wave, label='Clean Sine Wave')
plt.plot(t, fs_arr, label='Fourier sine wave')
plt.plot(t, noisy_sine_wave, label='Noisy Sine Wave')
plt.title('Sine Function with Phase Shift and Noise')
plt.xlabel('Time')
plt.ylabel('Amplitude')
plt.legend()
plt.show()
