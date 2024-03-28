import numpy
import matplotlib.cm as cm
import matplotlib.pyplot as plt
from keras.datasets import mnist
from keras.models import Sequential
from keras.layers import Dense, Activation
from keras import utils as np_utils

# download train and tests data together with corresponding
# labels
(X_train, Y_train), (X_test, Y_test) = mnist.load_data()

# reshape each sample (28x28) as a 784-pixel long array
X_train = X_train.reshape(60000, 784)
X_test = X_test.reshape(10000, 784)

# convert a digit value to one-hot encoded vector of zeroes
# and just one 1 in the entry corresponding to the digit:
# e.g. 4 is mapped to [0, 0, 0, 0, 1, 0, 0, 0, 0, 0]
classes = 10
Y_train = np_utils.to_categorical(Y_train, classes)
Y_test = np_utils.to_categorical(Y_test, classes)

input_size = 784
batch_size = 100
hidden_neurons = 100
epochs = 100

# define network
model = Sequential([
    Dense(hidden_neurons, input_dim=input_size),
    Activation('sigmoid'),
    Dense(classes),
    Activation('softmax')
])

# define cost function (loss), its optimizaiton - cross-entropy
# and stochastic gradient descent
model.compile(loss='categorical_crossentropy',
              metrics=['accuracy'], optimizer='sgd')

# train the network
model.fit(X_train, Y_train, batch_size=batch_size, epochs=epochs,
          verbose=1)

# evaluat the netowrk on the test data
score = model.evaluate(X_test, Y_test, verbose=1)
print('Test accuracy: ', score[1])

# visualise the weights
weights = model.layers[0].get_weights()
fig = plt.figure()
w = weights[0].T
for neuron in range(hidden_neurons):
    ax = fig.add_subplot(10, 10, neuron + 1)
    ax.axis("off")
    ax.imshow(numpy.reshape(w[neuron], (28, 28)), cmap=cm.Greys_r)
plt.savefig("neuron_images.png", dpi=300)
plt.show()
