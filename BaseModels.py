from pydantic import BaseModel
from typing import Optional
from passlib.context import CryptContext
from datetime import date, datetime, timedelta

class Login(BaseModel):

    username: str
    password: str
class Item(BaseModel):

    name: str
    amount: str
    id: str
    location: str

class Item1(BaseModel):

    id: str
    amount: str    
class Item2(BaseModel):

    id: str

class Item3(BaseModel):

    id: str
    location: str

class ID(BaseModel):

    id: str

class Name(BaseModel):

    name: str