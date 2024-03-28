Links:
Books: https://github.com/hiddenJuliet/pythondocument/tree/master
# Variables and passing parameters
Python passes arguments neither by reference nor by value, but by assignment. That is, when you call a Python function, each function argument becomes a variable to which the passed value is assigned. Python has no way of passing values by reference.
- If the assignment target is an identifier, or variable name, then this name is bound to the object. For example, in x = 2, x is the name and 2 is the object.
- If the name is already bound to a separate object, then it’s re-bound to the new object. For example, if x is already 2 and you issue x = 3, then the variable name x is re-bound to 3.
- If you pass a mutable object into a method, the method gets a reference to that same object and you can mutate it to your heart's delight, but if you rebind the reference in the method, the outer scope will know nothing about it, and after you're done, the outer reference will still point at the original object.
- If you pass an immutable object to a method, you still can't rebind the outer reference, and you can't even mutate the object.
Python variables are like reference variables in Java, so it’s better to think of them as labels attached to objects.
Python passes parameters using 'call by sharing' technique. In other words, the parameters inside the function become aliases of the actual arguments.
# Class instance attributes
- They are stored in __dict__ by default:
```
class A:
    def __init__(self):
        self.a = 0
        self.__c = 'test'
a = A()
print(a.__dict__)
{'a': 0, '_A__c': 'test'}
```
- instance attributes starting with double underscore have special meaning: they are mangled.
- Class instance attributes can be stored in a `tuple` instead of `dict` to safe some memory (if you have millions of elements then that might be important) - use `__slots__`:
```
class Vector2d:
    __slots__ = ('__x', '__y')
```
Problems with slot:
- You must remember to redeclare __slots__ in each subclass, because the inherited attribute is ignored by the interpreter.
- Instances will only be able to have the attributes listed in __slots__, unless you include '__dict__' in __slots__ (but doing so may negate the memory savings).
- Instances cannot be targets of weak references unless you remember to include '__weakref__' in __slots__.
## Notes
- object has identity, type and value
- `==` operator compares values (object's content)
- `is`/`is not` operator compares labes (if they're pointing the same object)
- every object has identity `id()` which never changes, the `is` operator compares identities
- In CPython, `id()` is a memory address, however, this is not defined, it can be something else
- when use `is`: `is None`, `is not None`
- `is` operator cannot be overloaded so it is faster than `==`
- `a == b` is syntactic sugar for `a.__eq__(b)`. By default `__eq__` is inherited from `object` and compares `Id`s (as `is` operator)
- usually `__eq__` is overriden
- copies as shallow by default (labels are copied, not underlying objects)
-- `l1 = [...] \ l2 = l1 \ l1 is l2 ==> True`
-- For copy list objects a contructor might be used e.g. `l1 = [...] \ l2 = list(l1) \ l1 is l2 ==> false`
-- For copying list objects - a shortcut: `l2 = l1[:]`. Slices does a copy of list object. Exception: for a tuple this `[:]` returns a reference! Also `tuple(t)` returns a reference. Similar behavior can be observed with instances of `str`, `bytes`, and `frozenset`.
-- Shallow copy because list's items (ref to objects) are copied but underlying objects not
-- Shallow copy of immutable items has no side effects
- `copy` module
-- `copy.copy(obj)` - shallow copy: new object, items copied in a shallow way
-- `copy.deepcopy(obj)` - deep copy: new object, new objects for items as well
- Mutable types as parameter defaults: BAD IDEA! (e.g. `def __init__(self, passengers=[])`). Default values are evaluated when the module is loaded.
- `del` removes only the reference to the object, it doesn't touch object itself
# List Comprehensions and Generator Expressions
Example of the listcomp: `codes = [ord(symbol) for symbol in symbols if ord(s) > 127]`
Also, cartesian product: `tshirts = [(color, size) for color in colors for size in sizes]`
Generators:
```
>>> symbols = '$¢£¥€¤'
>>> tuple(ord(symbol) for symbol in symbols)
(36, 162, 163, 165, 8364, 164)
>>> import array
>>> array.array('I', (ord(symbol) for symbol in symbols))
array('I', [36, 162, 163, 165, 8364, 164])
```
# Collections
## List
Handy type, keeps objects. Might not be suitable for millions of elements.
`l = [1, 2, 3]`
## Array
```
from array import array
floats = array('d', (random() for i in range(10**8)))
```
## Deque
```
>>> from collections import deque
>>> dq = deque(range(10), maxlen=10)
>>> dq
deque([0, 1, 2, 3, 4, 5, 6, 7, 8, 9], maxlen=10)
>>> dq.rotate(3)
>>> dq
deque([7, 8, 9, 0, 1, 2, 3, 4, 5, 6], maxlen=10)
>>> dq.rotate(-4)
>>> dq
deque([1, 2, 3, 4, 5, 6, 7, 8, 9, 0], maxlen=10)
>>> dq.appendleft(-1)
>>> dq
deque([-1, 1, 2, 3, 4, 5, 6, 7, 8, 9], maxlen=10)
>>> dq.extend([11, 22, 33])
>>> dq
deque([3, 4, 5, 6, 7, 8, 9, 11, 22, 33], maxlen=10)
>>> dq.extendleft([10, 20, 30, 40])
>>> dq
deque([40, 30, 20, 10, 3, 4, 5, 6, 7, 8], maxlen=10)
```
## Dictionary
```
>>> a = dict(one=1, two=2, three=3)
>>> b = {'one': 1, 'two': 2, 'three': 3}
>>> c = dict(zip(['one', 'two', 'three'], [1, 2, 3]))
>>> d = dict([('two', 2), ('one', 1), ('three', 3)])
>>> e = dict({'three': 3, 'one': 1, 'two': 2})
>>> a == b == c == d == e
True
```
dict Comprehensions
```
>>> DIAL_CODES = [
(86, 'China'),
(91, 'India'),
(81, 'Japan'),
]
>>> country_code = {country: code for code, country in DIAL_CODES}
```
## Closures
```
def make_averager():
    # series is a 'free variable'
    series = []
    def averager(new_value):
        series.append(new_value) # list is mutable so it's possible to do that
        total = sum(series)
        return total/len(series)
    return averager
>>> avg = make_averager()
>>> avg(10)
10.0
>>> avg(11)
10.5
>>> avg(12)
11.0
```
However, this will not work:
```
def make_averager():
    count = 0
    total = 0
    def averager(new_value):
        count += 1 # That will fail as there is attempt
        # to assign a value to local variable (count = count + 1)
        # in other words count is treated as a local var because it's immutable
        # so this is not an update but rebinding to a different value
        total += new_value
        return total / count
    return averager
```
to fix the problem we need to use `nonlocal` key word:
```
def make_averager():
    count = 0
    total = 0
    def averager(new_value):
        nonlocal count, total
        count += 1
        total += new_value
        return total / count
    return averager
```
## Decorators
### Function decorators
```
registry = []

def register(func):
    print('running register(%s)' % func)
    registry.append(func)
    return func

@register
def f1():
    print('running f1()')
# When module is being imported: f1 = register(f1)
```
### Example decorator
```
import time
def clock(func):
    def clocked(*args): #
        t0 = time.perf_counter()
        result = func(*args) #
        elapsed = time.perf_counter() - t0
        name = func.__name__
        arg_str = ', '.join(repr(arg) for arg in args)
        print('[%0.8fs] %s(%s) -> %r' % (elapsed, name, arg_str, result))
        return result
    return clocked
```
### Example decorator that takes parameters
```
import time
DEFAULT_FMT = '[{elapsed:0.8f}s] {name}({args}) -> {result}'
def clock(fmt=DEFAULT_FMT):
    # the decorate function is actual decorator
    def decorate(func):
        def clocked(*_args):
            t0 = time.time()
            _result = func(*_args)
            elapsed = time.time() - t0
            name = func.__name__
            args = ', '.join(repr(arg) for arg in _args)
            result = repr(_result)
            print(fmt.format(**locals()))
            return _result
        return clocked
    return decorate
if __name__ == '__main__':
    @clock()
    def snooze(seconds):
        time.sleep(seconds)
    for i in range(3):
        snooze(.123)
```
### built-in property
### built-in classmethod
### built-in staticmethod
### functools.wrap
This decorator is used for building another decorators.
### functools.lru_cache
It implements memoization: an optimization technique that works by saving the results of previous invocations of an expensive function, avoiding repeat computations on previously used arguments.
```
import functools

@functools.lru_cache(maxsize=128, typed=False) #
def fibonacci(n):
    if n < 2:
        return n
    return fibonacci(n-2) + fibonacci(n-1)
```
### functools.singledispatch
Example:
```
from functools import singledispatch
from collections import abc
import numbers
import html

@singledispatch
def htmlize(obj):
    content = html.escape(repr(obj))
    return '<pre>{}</pre>'.format(content)

@htmlize.register(str)
def _(text):
    content = html.escape(text).replace('\n', '<br>\n')
    return '<p>{0}</p>'.format(content)

@htmlize.register(numbers.Integral)
def _(n):
    return '<pre>{0} (0x{0:x})</pre>'.format(n)

@htmlize.register(tuple)
@htmlize.register(abc.MutableSequence)
def _(seq):
    inner = '</li>\n<li>'.join(htmlize(item) for item in seq)
    return '<ul>\n<li>' + inner + '</li>\n</ul>'
```
## Debugging
https://docs.python.org/3/library/pdb.html
`import pdb; pdb.set_trace()` or `breakpoint()`
From the command line: `python3 -m pdb myscript.py`
