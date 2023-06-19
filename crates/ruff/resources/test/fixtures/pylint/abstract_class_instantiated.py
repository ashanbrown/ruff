"""
Test for abstract-class-instantiated
"""
import abc

class Animal(abc.ABC):
    @abc.abstractmethod
    def make_sound(self):
        pass

sheep = Animal()  # [abstract-class-instantiated]

class Mammal(metaclass=abc.ABCMeta):
    def __init__(self):
        pass

    @abc.abstractmethod
    def make_sound(self):
        pass


cow = Mammal()  # [abstract-class-instantiated]

class Vehicle(abc.ABC):
    pass

car = Vehicle()  # Vehicle can be instantiated since it has no abstract methods
