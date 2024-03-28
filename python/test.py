import sys
import os

t = ('a', 'b', 'c')
c = ('d', 'd')
d = t + c
print(d)
print("Hello world")
print(sys.version)
print(os.getcwd())
print("done")


l = [i for i in range(1, 10)]
l1 = l[1:4]
l2 = l[3:5]
print(l)
print(l1)
print(l2)
print('Mod')
l[3] = 0
print(l)
print(l1)
print(l2)

#########################
print('--------------------------------')


def tag(name, *content, cls=None, **attrs):
    # attrs: keyword parameters captured as dict
    if cls is not None:
        attrs['class'] = cls
        print('attrs ----> ', attrs)
    if attrs:
        attr_str = ''.join(' %s="%s"' % (attr, value)
                           for attr, value in sorted(attrs.items()))
    else:
        attr_str = ''
    if content:
        return '\n'.join('<%s%s>%s</%s>' % (name, attr_str, c, name)
                         for c in content)
    else:
        return '<%s%s />' % (name, attr_str)


print(tag('br'))
print(tag('p', 'hello'))
print(tag('p', 'hello', 'world'))
print(tag('p', 'hello', id=33))
print(tag('p', 'hello', 'world', cls='sidebar'))
print(tag(content='testing', name='img'))
mytag = {'name': 'img', 'title': 'Sunset Boulevard',
         'src': 'sunset.jpg', 'cls': 'framed'}
print(tag(**mytag))
print('-' * 10)


def foo(arg: 'int'):
    print(arg)


foo(10)
foo('a')
